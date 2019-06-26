import "mocha";
import chaiAsPromised = require("chai-as-promised");
import { use as chaiUse, expect } from "chai";
import BigNumber from "bignumber.js";
import { RpcClient } from "./core/rpc-client";
import { newRpcClient, newWalletRequest, sleep, RECEIVE_WALLET_ADDRESS, VIEW_WALLET_ADDRESS } from "./core/setup";
chaiUse(chaiAsPromised);

describe("Wallet balance", () => {
  let client: RpcClient;
  before(() => {
    client = newRpcClient();
  });

  it("User can retrieve correct balance in genesis distribution", () => {
    const walletRequest = newWalletRequest("Default");

    return expect(
      client.request("wallet_balance", [walletRequest])
    ).to.eventually.deep.eq(new BigNumber("2500000000000000000"));
  });

  it("User can retrieve correct balance after send funds", async () => {
    const walletRequest = newWalletRequest("Default");

    await expect(
      client.request("wallet_balance", [walletRequest])
    ).to.eventually.deep.eq(new BigNumber("2500000000000000000"));

    await client.request("wallet_sendtoaddress", [
      walletRequest,
      RECEIVE_WALLET_ADDRESS,
      500000000000000000
    ]);

    await sleep(2000);

    return expect(
      client.request("wallet_balance", [walletRequest])
    ).to.eventually.deep.eq(new BigNumber("2000000000000000000"));
  });

  it("User can retrieve correct balance after receive funds", async () => {
    const spendWalletRequest = newWalletRequest("Spend");
    const viewWalletRequest = newWalletRequest("View");

    await expect(
      client.request("wallet_balance", [viewWalletRequest])
    ).to.eventually.deep.eq(new BigNumber("3000000000000000000"));

    const amountToSpend = 500000000000000000;
    await client.request("wallet_sendtoaddress", [
      spendWalletRequest,
      VIEW_WALLET_ADDRESS,
      amountToSpend
    ]);

    await sleep(2000);

    return expect(
      client.request("wallet_balance", [viewWalletRequest])
    ).to.eventually.deep.eq(new BigNumber("3000000000000000000").plus(amountToSpend));
  });
});
