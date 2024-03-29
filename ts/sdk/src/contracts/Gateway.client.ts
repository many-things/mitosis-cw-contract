/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Addr, HexBinary, InstantiateMsg, ExecuteMsg, CosmosMsgForEmpty, BankMsg, Uint128, StakingMsg, DistributionMsg, Binary, IbcMsg, Timestamp, Uint64, WasmMsg, GovMsg, VoteOption, Coin, Empty, IbcTimeout, IbcTimeoutBlock, QueryMsg, ConfigResponse } from "./Gateway.types";
export interface GatewayReadOnlyInterface {
  contractAddress: string;
  getConfig: () => Promise<ConfigResponse>;
}
export class GatewayQueryClient implements GatewayReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getConfig = this.getConfig.bind(this);
  }

  getConfig = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_config: {}
    });
  };
}
export interface GatewayInterface extends GatewayReadOnlyInterface {
  contractAddress: string;
  sender: string;
  changeOwner: ({
    newOwner,
    newPublicKey
  }: {
    newOwner: Addr;
    newPublicKey: HexBinary;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  changeLiquidityManager: ({
    newLiquidityManager
  }: {
    newLiquidityManager: Addr;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  changeDenomManager: ({
    newDenomManager
  }: {
    newDenomManager: Addr;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  pause: ({
    expiresAt
  }: {
    expiresAt: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  send: ({
    opArgs,
    opId
  }: {
    opArgs: string[];
    opId: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  execute: ({
    msgs,
    signature
  }: {
    msgs: CosmosMsgForEmpty[];
    signature: HexBinary;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  release: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class GatewayClient extends GatewayQueryClient implements GatewayInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.changeOwner = this.changeOwner.bind(this);
    this.changeLiquidityManager = this.changeLiquidityManager.bind(this);
    this.changeDenomManager = this.changeDenomManager.bind(this);
    this.pause = this.pause.bind(this);
    this.send = this.send.bind(this);
    this.execute = this.execute.bind(this);
    this.release = this.release.bind(this);
  }

  changeOwner = async ({
    newOwner,
    newPublicKey
  }: {
    newOwner: Addr;
    newPublicKey: HexBinary;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_owner: {
        new_owner: newOwner,
        new_public_key: newPublicKey
      }
    }, fee, memo, funds);
  };
  changeLiquidityManager = async ({
    newLiquidityManager
  }: {
    newLiquidityManager: Addr;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_liquidity_manager: {
        new_liquidity_manager: newLiquidityManager
      }
    }, fee, memo, funds);
  };
  changeDenomManager = async ({
    newDenomManager
  }: {
    newDenomManager: Addr;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      change_denom_manager: {
        new_denom_manager: newDenomManager
      }
    }, fee, memo, funds);
  };
  pause = async ({
    expiresAt
  }: {
    expiresAt: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      pause: {
        expires_at: expiresAt
      }
    }, fee, memo, funds);
  };
  send = async ({
    opArgs,
    opId
  }: {
    opArgs: string[];
    opId: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      send: {
        op_args: opArgs,
        op_id: opId
      }
    }, fee, memo, funds);
  };
  execute = async ({
    msgs,
    signature
  }: {
    msgs: CosmosMsgForEmpty[];
    signature: HexBinary;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      execute: {
        msgs,
        signature
      }
    }, fee, memo, funds);
  };
  release = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      release: {}
    }, fee, memo, funds);
  };
}