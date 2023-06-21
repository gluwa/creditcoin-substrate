import { BN } from "creditcoin-js";

export const MICROUNITS_PER_CTC = new BN("1000000000000000000");

export function parseCTCString(amount: string): BN {
  const CTC = amount.split(".")[0];
  const CTCinMicrounits = new BN(CTC).mul(MICROUNITS_PER_CTC);

  const decimal = amount.split(".")[1];
  const decimals = decimal.length;
  const decimalInMicrounits = new BN(decimal)
    .mul(MICROUNITS_PER_CTC)
    .div(new BN(10).pow(new BN(decimals)));

  return CTCinMicrounits.add(decimalInMicrounits);
}

export function toMicrounits(amount: number | BN): BN {
  return new BN(amount).mul(MICROUNITS_PER_CTC);
}

export function toCTCString(amount: BN): string {
  const CTC = amount.div(MICROUNITS_PER_CTC);
  const remainder = amount.mod(MICROUNITS_PER_CTC);
  const remainderString = remainder.toString().padStart(18, "0");
  return `${CTC}.${remainderString} CTC`;
}

export function readAmount(amount: string): BN {
  return new BN(amount);
}

export function readAmountFromHex(amount: string): BN {
  return new BN(amount.slice(2), 16);
}

export interface Balance {
  free: BN;
  reserved: BN;
  miscFrozen: BN;
  feeFrozen: BN;
}

export async function getBalance(address: string, api: any) {
  const account = await api.query.system.account(address);
  return balanceFromData(account.data);
}

function balanceFromData(data: any): Balance {
  return {
    free: data.free,
    reserved: data.reserved,
    miscFrozen: data.miscFrozen,
    feeFrozen: data.feeFrozen,
  };
}

export function printBalance(balance: Balance) {
  console.log("Available:", toCTCString(balance.free.sub(balance.miscFrozen)));
  console.log("Free:", toCTCString(balance.free));
  console.log("Reserved:", toCTCString(balance.reserved));
  console.log("Misc Frozen:", toCTCString(balance.miscFrozen));
  console.log("Fee Frozen:", toCTCString(balance.feeFrozen));
}
