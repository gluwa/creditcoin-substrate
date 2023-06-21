import { cryptoWaitReady } from "@polkadot/util-crypto";
import { Command } from "commander";
import { getCallerSeedFromEnvOrPrompt, initKeyringPair } from "../utils/account";

export function makeShowAddressCommand() {
  const cmd = new Command("show-address");
  cmd.description("Show account address");
  cmd.action(showAddressAction);
  return cmd;
}

async function showAddressAction() {
  await cryptoWaitReady();
  const seed = await getCallerSeedFromEnvOrPrompt();
  const pair = initKeyringPair(seed);
  const address = pair.address;

  console.log("Account address:", address);

  process.exit(0);
}
