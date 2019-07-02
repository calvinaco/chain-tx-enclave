import { RpcClient } from "./rpc-client";
import * as addressList from "../../../address-list.json";

export const DEFAULT_WALLET_ADDRESS = (<any>addressList).default;
export const SPEND_WALLET_ADDRESS = (<any>addressList).spend;
export const VIEW_WALLET_ADDRESS = (<any>addressList).view;
export const RECEIVE_WALLET_ADDRESS = (<any>addressList).receive;

const clientRpcPort = Number(process.env.CLIENT_RPC_ZERO_FEE_PORT) || 16659;

export const newRpcClient = (host: string = "localhost", port: number = clientRpcPort) => {
    return new RpcClient(`http://${host}:${port}`);
};

export const sleep = (ms: number = 1000) => {
  return new Promise(resolve => {
    setTimeout(resolve, ms);
  });
};

interface WalletRequest {
  name: string;
  passphrase: string;
}

export const generateWalletName = (prefix: string = "New Wallet"): string => {
  return `${prefix} ${Date.now()}`;
};

export const newWalletRequest = (
  name: string,
  passphrase: string = "uV97tEs5!*lLRQKj"
): WalletRequest => {
  return {
    name,
    passphrase
  };
};
