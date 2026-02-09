import dotenv from "dotenv";
dotenv.config();

import { DatabaseService } from "./services/database";
import { TokenMetadataService } from "./services/token-metadata";
import { EventIndexer } from "./services/event-indexer";
import { networks } from "./config/networks";
import { logger } from "./utils/logger";

async function sync() {
  logger.info("Starting sync...");

  const db = new DatabaseService();
  await db.initialize();

  const tokenMetadata = new TokenMetadataService(networks);
  const indexer = new EventIndexer(networks, db, tokenMetadata);

  for (const network of networks) {
    const savedNetwork = await db.getNetwork(network.chainId);
    // Use last synced block from DB, fall back to env config start block, then 0
    const fromBlock = (savedNetwork?.blockNumber && savedNetwork.blockNumber > 0) 
      ? savedNetwork.blockNumber + 1  // Start from next block after last synced
      : network.blockNumber;  // Use configured start block from env
    
    logger.info(`\nSyncing ${network.name} from block ${fromBlock}...`);
    await indexer.syncHistoricalEvents(network.chainId, fromBlock);
  }

  logger.info("\n✓ Sync complete");
  process.exit(0);
}

sync().catch((error) => {
  logger.error("Sync failed:", error);
  process.exit(1);
});