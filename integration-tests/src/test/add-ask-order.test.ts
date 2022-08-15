import { Guid, LoanTerms } from 'creditcoin-js';
import { KeyringPair } from 'creditcoin-js';
import { createCreditcoinLoanTerms } from 'creditcoin-js/lib/transforms';
import { AddressRegistered } from 'creditcoin-js/lib/extrinsics/register-address';
import { POINT_01_CTC } from '../constants';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { loanTermsWithCurrency, testData, tryRegisterAddress } from './common';
import { extractFee } from '../utils';

describe('AddAskOrder', () => {
    let ccApi: CreditcoinApi;
    let lender: KeyringPair;
    let lenderRegAddr: AddressRegistered;
    let askGuid: Guid;
    let loanTerms: LoanTerms;

    const { blockchain, expirationBlock, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        loanTerms = await loanTermsWithCurrency(ccApi);
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const lenderWallet = createWallet('lender');

        lenderRegAddr = await tryRegisterAddress(
            ccApi,
            lenderWallet.address,
            blockchain,
            signAccountId(ccApi.api, lenderWallet, lender.address),
            lender,
            (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
        );
        askGuid = Guid.newGuid();
    });

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .addAskOrder(
                    lenderRegAddr.itemId,
                    createCreditcoinLoanTerms(api, loanTerms),
                    expirationBlock,
                    askGuid.toString(),
                )
                .signAndSend(lender, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    });
});
