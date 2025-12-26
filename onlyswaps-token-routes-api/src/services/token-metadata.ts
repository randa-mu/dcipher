import { ethers } from "ethers";
import { TokenInfo } from "../types";
import { logger } from "../utils/logger";

const ERC20_ABI = [
  "function symbol() view returns (string)",
  "function name() view returns (string)",
  "function decimals() view returns (uint8)"
];

export class TokenMetadataService {
  private providers: Map<number, ethers.Provider> = new Map();

  constructor(networks: { chainId: number; rpcUrl: string }[]) {
    for (const network of networks) {
      const provider = new ethers.JsonRpcProvider(network.rpcUrl);
      this.providers.set(network.chainId, provider);
    }
  }

  async fetchTokenMetadata(tokenAddress: string, chainId: number): Promise<TokenInfo | null> {
    const provider = this.providers.get(chainId);
    if (!provider) {
      logger.error(`No provider for chain ${chainId}`);
      return null;
    }

    try {
      const contract = new ethers.Contract(tokenAddress, ERC20_ABI, provider);
      
      const [symbol, name, decimals] = await Promise.all([
        contract.symbol().catch(() => "UNKNOWN"),
        contract.name().catch(() => "Unknown Token"),
        contract.decimals().catch(() => 18)
      ]);

      return {
        address: tokenAddress,
        chainId,
        symbol,
        name,
        decimals: Number(decimals)
      };
    } catch (error) {
      logger.error(`Failed to fetch metadata for ${tokenAddress} on chain ${chainId}:`, error);
      return null;
    }
  }
}