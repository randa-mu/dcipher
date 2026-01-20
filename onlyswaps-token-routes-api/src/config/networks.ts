import { NetworkConfig } from "../types";

export const networks: NetworkConfig[] = [
  {
    chainId: 42161,
    name: "Arbitrum",
    rpcUrl: process.env.ARBITRUM_RPC_URL || "",
    routerAddress: process.env.ARBITRUM_ROUTER || "",
    blockNumber: Number(process.env.ARBITRUM_START_BLOCK || 0)
  },
  {
    chainId: 5042002,
    name: "Arc Testnet",
    rpcUrl: process.env.ARC_TESTNET_RPC_URL || "",
    routerAddress: process.env.ARC_TESTNET_ROUTER || "",
    blockNumber: Number(process.env.ARC_TESTNET_START_BLOCK || 0)
  },
  {
    chainId: 43113,
    name: "Avalanche Fuji",
    rpcUrl: process.env.AVALANCHE_FUJI_RPC_URL || "",
    routerAddress: process.env.AVALANCHE_FUJI_ROUTER || "",
    blockNumber: Number(process.env.AVALANCHE_FUJI_START_BLOCK || 0)
  },
  {
    chainId: 43114,
    name: "Avalanche",
    rpcUrl: process.env.AVALANCHE_RPC_URL || "",
    routerAddress: process.env.AVALANCHE_ROUTER || "",
    blockNumber: Number(process.env.AVALANCHE_START_BLOCK || 0)
  },
  {
    chainId: 8453,
    name: "Base",
    rpcUrl: process.env.BASE_RPC_URL || "",
    routerAddress: process.env.BASE_ROUTER || "",
    blockNumber: Number(process.env.BASE_START_BLOCK || 0)
  },
  {
    chainId: 84532,
    name: "Base Sepolia",
    rpcUrl: process.env.BASE_SEPOLIA_RPC_URL || "",
    routerAddress: process.env.BASE_SEPOLIA_ROUTER || "",
    blockNumber: Number(process.env.BASE_SEPOLIA_START_BLOCK || 0)
  },
  {
    chainId: 56,
    name: "Binance",
    rpcUrl: process.env.BINANCE_RPC_URL || "",
    routerAddress: process.env.BINANCE_ROUTER || "",
    blockNumber: Number(process.env.BINANCE_START_BLOCK || 0)
  },
  {
    chainId: 1,
    name: "Ethereum",
    rpcUrl: process.env.ETHEREUM_RPC_URL || "",
    routerAddress: process.env.ETHEREUM_ROUTER || "",
    blockNumber: Number(process.env.ETHEREUM_START_BLOCK || 0)
  },
  {
    chainId: 314,
    name: "Filecoin",
    rpcUrl: process.env.FILECOIN_RPC_URL || "",
    routerAddress: process.env.FILECOIN_ROUTER || "",
    blockNumber: Number(process.env.FILECOIN_START_BLOCK || 0)
  },
  {
    chainId: 314159,
    name: "Filecoin Calibration",
    rpcUrl: process.env.FILECOIN_CALIBRATION_RPC_URL || "",
    routerAddress: process.env.FILECOIN_CALIBRATION_ROUTER || "",
    blockNumber: Number(process.env.FILECOIN_CALIBRATION_START_BLOCK || 0)
  },
  {
    chainId: 59144,
    name: "Linea",
    rpcUrl: process.env.LINEA_RPC_URL || "",
    routerAddress: process.env.LINEA_ROUTER || "",
    blockNumber: Number(process.env.LINEA_START_BLOCK || 0)
  },
  {
    chainId: 10,
    name: "Optimism",
    rpcUrl: process.env.OP_RPC_URL || "",
    routerAddress: process.env.OP_ROUTER || "",
    blockNumber: Number(process.env.OP_START_BLOCK || 0)
  },
  {
    chainId: 534352,
    name: "Scroll",
    rpcUrl: process.env.SCROLL_RPC_URL || "",
    routerAddress: process.env.SCROLL_ROUTER || "",
    blockNumber: Number(process.env.SCROLL_START_BLOCK || 0)
  }
];