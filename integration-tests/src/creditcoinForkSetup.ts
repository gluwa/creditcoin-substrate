import { POINT_01_CTC } from 'creditcoin-js';
import { default as globalSetup } from './globalSetup';

const setup = async () => {
    // 0.006 CTC to account for lower fees on testnet/mainnet due to inactivity
    (global as any).CREDITCOIN_MINIMUM_TXN_FEE = 0.6 * POINT_01_CTC;

    // to satisfy the regexp in metrics.test.ts
    (global as any).CREDITCOIN_NETWORK_SHORT_NAME = '.*-fork';

    (global as any).CREDITCOIN_SWITCH_TO_POS_ALREADY_CALLED = process.env.SWITCH_TO_POS_ALREADY_CALLED === '1';

    await globalSetup();
};

export default setup;
