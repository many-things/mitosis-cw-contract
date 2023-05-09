/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Addr, Uint128, Coin, GetBalanceResponse, GetBondResponse, ConfigResponse, GetUnbondResponse, GetUnbondListResponse, PauseInfoResponse } from "./Liquiditymanager.types";
export interface LiquiditymanagerReadOnlyInterface {
    contractAddress: string;
    getConfig: () => Promise<ConfigResponse>;
    pauseInfo: () => Promise<PauseInfoResponse>;
    getBalance: ({ depositor }: {
        depositor: Addr;
    }) => Promise<GetBalanceResponse>;
    getBond: ({ bonder }: {
        bonder: Addr;
    }) => Promise<GetBondResponse>;
    getUnbond: ({ unbondId }: {
        unbondId: number;
    }) => Promise<GetUnbondResponse>;
    getUnbondsByOwner: ({ owner }: {
        owner: Addr;
    }) => Promise<GetUnbondListResponse>;
}
export declare class LiquiditymanagerQueryClient implements LiquiditymanagerReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    getConfig: () => Promise<ConfigResponse>;
    pauseInfo: () => Promise<PauseInfoResponse>;
    getBalance: ({ depositor }: {
        depositor: Addr;
    }) => Promise<GetBalanceResponse>;
    getBond: ({ bonder }: {
        bonder: Addr;
    }) => Promise<GetBondResponse>;
    getUnbond: ({ unbondId }: {
        unbondId: number;
    }) => Promise<GetUnbondResponse>;
    getUnbondsByOwner: ({ owner }: {
        owner: Addr;
    }) => Promise<GetUnbondListResponse>;
}
export interface LiquiditymanagerInterface extends LiquiditymanagerReadOnlyInterface {
    contractAddress: string;
    sender: string;
    deposit: ({ depositor }: {
        depositor?: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    withdraw: ({ amount, withdrawer }: {
        amount: Coin;
        withdrawer?: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    delegate: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    undelegate: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    bond: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    startUnbond: ({ amount }: {
        amount: Uint128;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    unbond: ({ unbondId }: {
        unbondId: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    changeOwner: ({ newOwner }: {
        newOwner: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    grantRole: ({ addr, role }: {
        addr: Addr;
        role: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    revokeRole: ({ addr, role }: {
        addr: Addr;
        role: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    pause: ({ expiresAt }: {
        expiresAt: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    release: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    changeConfig: ({ unbondingPeriod }: {
        unbondingPeriod: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class LiquiditymanagerClient extends LiquiditymanagerQueryClient implements LiquiditymanagerInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    deposit: ({ depositor }: {
        depositor?: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    withdraw: ({ amount, withdrawer }: {
        amount: Coin;
        withdrawer?: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    delegate: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    undelegate: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    bond: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    startUnbond: ({ amount }: {
        amount: Uint128;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    unbond: ({ unbondId }: {
        unbondId: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    changeOwner: ({ newOwner }: {
        newOwner: Addr;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    grantRole: ({ addr, role }: {
        addr: Addr;
        role: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    revokeRole: ({ addr, role }: {
        addr: Addr;
        role: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    pause: ({ expiresAt }: {
        expiresAt: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    release: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    changeConfig: ({ unbondingPeriod }: {
        unbondingPeriod: number;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=Liquiditymanager.client.d.ts.map