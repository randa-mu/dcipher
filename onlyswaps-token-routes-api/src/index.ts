import dotenv from "dotenv";
dotenv.config();

import { DatabaseService } from "./services/database";
import { TokenMetadataService } from "./services/token-metadata";
import { EventIndexer } from "./services/event-indexer";
import { createServer } from "./api/server";
import { networks } from "./config/networks";
import { logger } from "./utils/logger";

async function main() {
  logger.info("🚀 Starting Only Swaps Indexer...");

  // Initialize database
  const db = new DatabaseService();
  await db.initialize();
  logger.info("✓ Database initialized");

  // Insert/update networks in DB
  for (const network of networks) {
    await db.upsertNetwork({
      chainId: network.chainId,
      name: network.name,
      routerAddress: network.routerAddress,
      blockNumber: network.blockNumber
    });
  }
  logger.info("✓ Networks configured");

  const tokenMetadata = new TokenMetadataService(networks);
  const indexer = new EventIndexer(networks, db, tokenMetadata);

  //
  // --- Start API server IMMEDIATELY ---
  //
  const app = createServer(db);
  const PORT = process.env.PORT || 3000;

  app.listen(PORT, () => {
    logger.info(`🌐 API server running on http://localhost:${PORT}`);
    logger.info("Endpoints:");
    logger.info("  GET  /health");
    logger.info("  GET  /api/networks");
    logger.info("  GET  /api/networks/:chainId");
    logger.info("  GET  /api/mappings");
    logger.info("  GET  /api/mappings?srcChainId=1&dstChainId=137");
    logger.info("  GET  /api/mappings/token/:address/:chainId");
    logger.info("  GET  /api/tokens/:address/:chainId");
  });

  //
  // --- Sync each chain sequentially ---
  for (const network of networks) {

    const chainPrefix = `[${network.name}][${network.chainId}]`;
    logger.info(`${chainPrefix} --- Starting sync ---`);

    // If config start block is zero, skip historical sync and ignore DB block
    if (network.blockNumber === 0) {
      logger.info(`${chainPrefix} Config start block is zero → skipping historical sync.`);
    } else {
      const savedNetwork = await db.getNetwork(network.chainId);
      let fromBlock: number;

      // Determine starting block
      if (savedNetwork?.blockNumber && savedNetwork.blockNumber > 0) {
        fromBlock = savedNetwork.blockNumber + 1;
      } else if (network.blockNumber > 0) {
        fromBlock = network.blockNumber;
      } else {
        const provider = indexer.getProvider(network.chainId);
        const latestBlock = await provider.getBlockNumber();
        fromBlock = latestBlock;
        logger.info(`${chainPrefix} No start block configured → using latest block ${latestBlock}`);
      }

      logger.info(`${chainPrefix} Syncing from block ${fromBlock}...`);

      try {
        await indexer.syncHistoricalEvents(network.chainId, fromBlock);
        logger.info(`${chainPrefix} ✓ Historical sync finished`);
      } catch (err) {
        logger.error(`${chainPrefix} ❌ Historical sync failed:`, err);
      }
    }

    // Start live event listener
    try {
      await indexer.startListening(network.chainId);
      logger.info(`${chainPrefix} 👂 Listening for new events...`);
    } catch (err) {
      logger.error(`${chainPrefix} ❌ Failed to start listener:`, err);
    }
  }
}

main().catch((error) => {
  logger.error("Application crashed:", error);
  process.exit(1);
});


