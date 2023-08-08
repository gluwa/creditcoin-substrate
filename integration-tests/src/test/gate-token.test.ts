import { KeyringPair, creditcoinApi, Keyring } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { checkAddress, testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, testIf } from '../utils';

import { deployGATEToken } from '../ctc-deploy';
import { JsonRpcProvider } from '@ethersproject/providers';
import { Signer, Wallet, Contract } from 'ethers';
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { personalSignAccountId, signAccountId } from 'creditcoin-js/lib/utils';
import { ethSignSignature, personalSignSignature } from 'creditcoin-js/lib/extrinsics/register-address-v2';
import { requestCollectCoins } from 'creditcoin-js/lib/extrinsics/request-collect-coins';

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

    // Holds the reference to the deployed GATE contract
    let gateToken: any;
    let gateKeyring = new Keyring({ type: 'ed25519', ss58Format: 3 });
    let gateFaucet = gateKeyring.addFromUri(mnemonicGenerate(12));

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
        const { api, extrinsics: { registerAddressV2, requestSwapGATE } } = ccApi;

        // transfer some CTC to the on-chain burn GATE faucet
        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(gateFaucet.address, 1000, 0))
            .signAndSend(sudoSigner, { nonce: -1 });

        // Set the on chain location for the burn contract to be the address of the deployer wallet
        const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsGateContract', {
            address: gateToken.address,
            chain: testingData.blockchain,
        });
        await api.tx.sudo
            .sudo(api.tx.creditcoin.setBurnGateContract(contract))
            .signAndSend(sudoSigner, { nonce: -1 });


        // Set the faucet address in onchain storage to the one that we transfered ctc to earlier
        // The sp_core::ecsda::Public type exposed by the extrinsic expects a buffer of size 33. Probably a better way to do this
        let addr_buffer = new Uint8Array(gateFaucet.addressRaw.length + 1);
        addr_buffer.set(gateFaucet.addressRaw)
        await api.tx.sudo
            .sudo(api.tx.creditcoin.setBurnGateFaucetAddress(addr_buffer))
            .signAndSend(sudoSigner, { nonce: -1 })

        const mintTx = await gateToken.mint(deployer.address, 1000)
        await mintTx.wait(3);
        const balance = await gateToken.balanceOf(deployer.address);
        expect(balance.eq(1000)).toBe(true);


        const burnTx = await gateToken.burn(200);
        await burnTx.wait(3);

        const accountId = await signAccountId(api, deployer, sudoSigner.address);
        const proof = ethSignSignature(accountId);
        const lenderRegisteredAddress = await registerAddressV2(deployer.address, testingData.blockchain, proof, sudoSigner);


        const swapGATEEvent = await requestSwapGATE(lenderRegisteredAddress.item.externalAddress, sudoSigner, burnTx.hash)
        const swapGATEVerified = await swapGATEEvent.waitForVerification(800_000).catch();

        expect(swapGATEVerified).toBeTruthy();

    }, 900_000)
});


