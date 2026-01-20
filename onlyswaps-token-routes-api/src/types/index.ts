export interface NetworkConfig {
  chainId: number;
  name: string;
  rpcUrl: string;
  routerAddress: string;
  blockNumber: number;
}

export interface NetworkState {
  chainId: number;
  name: string;
  routerAddress: string;
  blockNumber: number;
}

export interface TokenInfo {
  address: string;
  chainId: number;
  symbol?: string;
  name?: string;
  decimals?: number;
}

export interface TokenMapping {
  id: string;
  srcTokenAddress: string;
  srcChainId: number;
  dstTokenAddress: string;
  dstChainId: number;
  isActive: boolean;
  blockNumber: number;
  txHash: string;
  timestamp: number;
}

export interface Database {
  networks: NetworkState[];
  tokens: TokenInfo[];
  mappings: TokenMapping[];
}