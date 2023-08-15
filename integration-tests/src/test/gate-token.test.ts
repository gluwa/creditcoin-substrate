import { KeyringPair, creditcoinApi, Keyring, BN, Balance } from 'creditcoin-js';
import { AccountId, Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi, VerificationError } from 'creditcoin-js/lib/types';
import { checkAddress, testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, testIf } from '../utils';

import { deployGATEToken } from '../ctc-deploy';
import { JsonRpcProvider } from '@ethersproject/providers';
import { Signer, Wallet, Contract } from 'ethers';
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { personalSignAccountId, signAccountId } from 'creditcoin-js/lib/utils';
import { ethSignSignature, personalSignSignature } from 'creditcoin-js/lib/extrinsics/register-address-v2';
import { requestCollectCoins } from 'creditcoin-js/lib/extrinsics/request-collect-coins';
import { GATEContract } from 'creditcoin-js/lib/extrinsics/request-collect-coins-v2';

describe('Test GATE Token', (): void => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;

    // Needed to interact with the ethererum private node
    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { keyring } = testingData;
    const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
    const deployer = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
    const burnAmount = 200;

    // Holds the reference to the deployed GATE contract
    let gateToken: any;
    let gateKeyring = new Keyring({ type: 'ed25519', ss58Format: 3 });
    let gateFaucet = gateKeyring.addFromUri(mnemonicGenerate(12));

    let gateKeyring2 = new Keyring({ type: 'ed25519', ss58Format: 3 });
    let gateFaucet2 = gateKeyring2.addFromUri(mnemonicGenerate(12));

    // the eth wallet that initiates the burn transaction on its own supply of GATE
    const burnerWallet = Wallet.createRandom({ provider: provider });

    beforeAll(async () => {
        gateToken = await deployGATEToken(deployer);

        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        }

    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    test('End to end', async () => {

        const { api, extrinsics: { registerAddressV2, requestCollectCoinsV2 } } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(gateFaucet.address, 1000, 0))
            .signAndSend(sudoSigner, { nonce: -1 });

        // Set the on chain location for the burn contract to be the address of the deployer wallet
        const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
            address: gateToken.address,
            chain: testingData.blockchain,
        });
        await api.tx.sudo
            .sudo(api.tx.creditcoin.setBurnGateContract(contract))
            .signAndSend(sudoSigner, { nonce: -1 });


        const mintTx = await gateToken.mint(deployer.address, 2500)
        await mintTx.wait(3);
        const balance = await gateToken.balanceOf(deployer.address);
        expect(balance.eq(2500)).toBe(true);


        const burnTx = await gateToken.burn(burnAmount);
        await burnTx.wait(3);

        const accountId = await signAccountId(api, deployer, sudoSigner.address);
        const proof = ethSignSignature(accountId);
        const lenderRegisteredAddress = await registerAddressV2(deployer.address, testingData.blockchain, proof, sudoSigner);

        const gateContract = GATEContract(lenderRegisteredAddress.item.externalAddress, burnTx.hash);

        // Test #1: The extrinsic should erorr when the faucet address has not been set
        await expect(
            requestCollectCoinsV2(
                gateContract,
                sudoSigner,
            ),
        ).rejects.toThrow(
            'creditcoin.BurnGATEFaucetNotSet',
        );

        await api.tx.sudo
            .sudo(api.tx.creditcoin.setBurnGateFaucetAddress(gateFaucet.address))
            .signAndSend(sudoSigner, { nonce: -1 })

        const swapGATEEvent = await requestCollectCoinsV2(gateContract, sudoSigner);
        const swapGATEVerified = await swapGATEEvent.waitForVerification(800_000).catch();

        // Test #2: This is a successful transfer and should proceed normally
        expect(swapGATEVerified).toBeTruthy();

        // Test #3: GATE -> CTC should be swapped in a 2:1 ratio
        expect(swapGATEVerified.amount.toNumber()).toEqual(burnAmount / 2);

        // Test #4: You cannot resubmit previously used burn transactions
        await expect(
            requestCollectCoinsV2(
                gateContract,
                sudoSigner,
            ),
        ).rejects.toThrow(
            'creditcoin.CollectCoinsAlreadyRegistered: The coin collection has already been registered',
        );
    }, 900_000)
});
