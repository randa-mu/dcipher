import { ethers } from "ethers";
import { DatabaseService } from "./database";
import { TokenMetadataService } from "./token-metadata";
import { NetworkConfig } from "../types";
import { logger } from "../utils/logger";

const ROUTER_ABI = [
  "event TokenMappingAdded(uint256 indexed dstChainId, address indexed dstToken, address indexed srcToken)",
  "event TokenMappingRemoved(uint256 indexed dstChainId, address indexed dstToken, address indexed srcToken)"
];

export class EventIndexer {
  private providers: Map<number, ethers.Provider> = new Map();
  private contracts: Map<number, ethers.Contract> = new Map();
  private networkNames: Map<number, string> = new Map();

  constructor(
    private networks: NetworkConfig[],
    private db: DatabaseService,
    private tokenMetadata: TokenMetadataService
  ) {
    this.initializeNetworks();
  }

  private getNetworkName(chainId: number): string {
    return this.networkNames.get(chainId) || `Chain ${chainId}`;
  }

  public getProvider(chainId: number): ethers.Provider {
    const provider = this.providers.get(chainId);
    if (!provider) {
      throw new Error(`Provider not found for chain ${chainId}`);
    }
    return provider;
  }

  private initializeNetworks() {
    for (const network of this.networks) {
      const provider = new ethers.JsonRpcProvider(network.rpcUrl);
      const contract = new ethers.Contract(network.routerAddress, ROUTER_ABI, provider);

      this.providers.set(network.chainId, provider);
      this.contracts.set(network.chainId, contract);
      this.networkNames.set(network.chainId, network.name);
    }
  }

  async syncHistoricalEvents(chainId: number, fromBlock: number = 0) {
    const contract = this.contracts.get(chainId);
    const provider = this.providers.get(chainId);
    
    if (!contract || !provider) {
      throw new Error(`Contract or provider not found for chain ${chainId}`);
    }

    const currentBlock = await provider.getBlockNumber();
    const networkName = this.getNetworkName(chainId);
    const totalBlocks = currentBlock - fromBlock;
    logger.info(`[${networkName}] Syncing from block ${fromBlock} to ${currentBlock} (${totalBlocks} blocks)`);

    const chunkSize = Number(process.env.BLOCK_CHUNK_SIZE) || 10000;
    
    for (let start = fromBlock; start < currentBlock; start += chunkSize) {
      const end = Math.min(start + chunkSize - 1, currentBlock);
      
      logger.info(`[${networkName}] Processing blocks ${start} to ${end}`);
      
      // Get TokenMappingAdded events - query with event signature topic
      // The signature hash should match: 0x00f6b276aebfc163c3646a63a0286f845fdd2df56be91b61b62044067ce849a4
      const addedEventSignature = "0x00f6b276aebfc163c3646a63a0286f845fdd2df56be91b61b62044067ce849a4";
      const addedEvents = await provider.getLogs({
        address: contract.target,
        topics: [addedEventSignature],
        fromBlock: start,
        toBlock: end
      });
      
      if (addedEvents.length > 0) {
        logger.info(`[${networkName}] Found ${addedEvents.length} TokenMappingAdded event(s)`);
        // Log first event to debug
        const first = addedEvents[0];
        logger.info(`[${networkName}] First event found: ${JSON.stringify({
          topics: first.topics,
          data: first.data,
          topicsLength: first.topics.length
        })}`);
      }
      
      for (const event of addedEvents) {
        try {
          // Check if data field has the parameters instead of topics
          if (event.data && event.data !== '0x' && event.data.length > 2) {
            // Parameters are in data field, not indexed
            logger.info(`[${networkName}] Decoding from data field: ${event.data}`);
            // Data contains: dstChainId (32 bytes), dstToken (32 bytes), srcToken (32 bytes)
            const dstChainId = BigInt('0x' + event.data.slice(2, 66));
            const dstToken = ethers.getAddress('0x' + event.data.slice(90, 130));
            const srcToken = ethers.getAddress('0x' + event.data.slice(154, 194));
            
            const eventWithArgs = {
              args: { dstChainId, dstToken, srcToken },
              blockNumber: event.blockNumber,
              transactionHash: event.transactionHash
            };
            
            await this.handleTokenMappingAdded(chainId, eventWithArgs);
            continue;
          }
          
          // All parameters are indexed, so they're in topics[1], topics[2], topics[3]
          // topics[0] is the event signature
          if (event.topics.length < 4) {
            logger.warn(`[${networkName}] Event missing topics: ${event.topics.length}, topics: ${JSON.stringify(event.topics)}`);
            continue;
          }
          
          // Manually decode indexed parameters from topics
          const dstChainId = BigInt(event.topics[1]);
          const dstToken = ethers.getAddress('0x' + event.topics[2].slice(26)); // Remove padding
          const srcToken = ethers.getAddress('0x' + event.topics[3].slice(26)); // Remove padding
          
          const eventWithArgs = {
            args: {
              dstChainId,
              dstToken,
              srcToken
            },
            blockNumber: event.blockNumber,
            transactionHash: event.transactionHash
          };
          
          await this.handleTokenMappingAdded(chainId, eventWithArgs);
        } catch (error) {
          logger.error(`[${networkName}] Failed to handle TokenMappingAdded event: ${error}`);
          // Continue processing other events
        }
      }

      // Get TokenMappingRemoved events
      const removedEvents = await provider.getLogs({
        address: contract.target,
        topics: [ethers.id("TokenMappingRemoved(uint256,address,address)")],
        fromBlock: start,
        toBlock: end
      });
      
      if (removedEvents.length > 0) {
        logger.info(`[${networkName}] Found ${removedEvents.length} TokenMappingRemoved event(s)`);
      }
      
      for (const event of removedEvents) {
        try {
          // All parameters are indexed, so they're in topics[1], topics[2], topics[3]
          if (event.topics.length < 4) {
            logger.warn(`[${networkName}] Event missing topics: ${event.topics.length}`);
            continue;
          }
          
          // Manually decode indexed parameters from topics
          const dstChainId = BigInt(event.topics[1]);
          const dstToken = ethers.getAddress('0x' + event.topics[2].slice(26)); // Remove padding
          const srcToken = ethers.getAddress('0x' + event.topics[3].slice(26)); // Remove padding
          
          const eventWithArgs = {
            args: {
              dstChainId,
              dstToken,
              srcToken
            },
            blockNumber: event.blockNumber,
            transactionHash: event.transactionHash
          };
          
          await this.handleTokenMappingRemoved(chainId, eventWithArgs);
        } catch (error) {
          logger.error(`[${networkName}] Failed to handle TokenMappingRemoved event: ${error}`);
          // Continue processing other events
        }
      }

      // Update progress after each chunk
      await this.db.updateNetworkBlockNumber(chainId, end);
      
      // Add delay between chunks to avoid rate limiting
      const delayMs = Number(process.env.REQUEST_DELAY_MS) || 200;
      if (delayMs > 0) {
        await this.sleep(delayMs);
      }
    }

    logger.info(`[${networkName}] Sync complete. Last block: ${currentBlock}`);
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  async startListening(chainId: number) {
    const contract = this.contracts.get(chainId);
    if (!contract) {
      throw new Error(`Contract not found for chain ${chainId}`);
    }

    const networkName = this.getNetworkName(chainId);

    contract.on("TokenMappingAdded", async (dstChainId, dstToken, srcToken, event) => {
      logger.info(`[${networkName}] TokenMappingAdded: ${srcToken} -> ${dstToken} (chain ${dstChainId})`);
      await this.handleTokenMappingAdded(chainId, event);
    });

    contract.on("TokenMappingRemoved", async (dstChainId, dstToken, srcToken, event) => {
      logger.info(`[${networkName}] TokenMappingRemoved: ${srcToken} -> ${dstToken} (chain ${dstChainId})`);
      await this.handleTokenMappingRemoved(chainId, event);
    });

    logger.info(`[${networkName}] Listening for events...`);
  }

  private async handleTokenMappingAdded(srcChainId: number, event: any) {
    try {
      const { dstChainId, dstToken, srcToken } = event.args;
      const blockNumber = event.blockNumber;
      const txHash = event.transactionHash;

      logger.info(`Processing mapping: ${srcToken} -> ${dstToken} (chain ${dstChainId}) at block ${blockNumber}`);

      // Fetch and store token metadata
      logger.info(`Fetching metadata for src token ${srcToken} on chain ${srcChainId}`);
      await this.fetchAndStoreTokenMetadata(srcToken, srcChainId);
      
      logger.info(`Fetching metadata for dst token ${dstToken} on chain ${dstChainId}`);
      await this.fetchAndStoreTokenMetadata(dstToken, Number(dstChainId));

      // Store mapping
      logger.info(`Storing mapping in database...`);
      await this.db.addTokenMapping({
        srcTokenAddress: srcToken,
        srcChainId: srcChainId,
        dstTokenAddress: dstToken,
        dstChainId: Number(dstChainId),
        blockNumber: blockNumber,
        txHash: txHash,
        isActive: true
      });

      logger.info(`✓ Successfully stored mapping: ${srcToken} -> ${dstToken}`);
    } catch (error) {
      logger.error(`Error handling TokenMappingAdded: ${error}`);
      throw error;
    }
  }

  private async handleTokenMappingRemoved(srcChainId: number, event: any) {
    const { dstChainId, dstToken, srcToken } = event.args;

    await this.db.removeTokenMapping({
      srcTokenAddress: srcToken,
      srcChainId: srcChainId,
      dstTokenAddress: dstToken,
      dstChainId: Number(dstChainId)
    });
  }

  private async fetchAndStoreTokenMetadata(tokenAddress: string, chainId: number) {
    try {
      const metadata = await this.tokenMetadata.fetchTokenMetadata(tokenAddress, chainId);
      if (metadata) {
        await this.db.upsertToken(metadata);
      }
    } catch (error) {
      logger.warn(`Failed to fetch metadata for ${tokenAddress} on chain ${chainId}, continuing anyway: ${error}`);
      // Don't throw - we still want to store the mapping even if metadata fetch fails
    }
  }
}