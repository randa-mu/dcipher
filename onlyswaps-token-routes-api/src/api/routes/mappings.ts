import { Router } from "express";
import { DatabaseService } from "../../services/database";
import { logger } from "../../utils/logger";

export function createMappingsRouter(db: DatabaseService) {
  const router = Router();

  router.get("/", async (req, res) => {
    try {
      const { srcChainId, dstChainId } = req.query;
      
      const mappings = await db.getTokenMappings(
        srcChainId ? Number(srcChainId) : undefined,
        dstChainId ? Number(dstChainId) : undefined
      );

      // Enrich with token metadata
      const enrichedMappings = await Promise.all(
        mappings.map(async (mapping) => {
          const srcToken = await db.getToken(mapping.srcTokenAddress, mapping.srcChainId);
          const dstToken = await db.getToken(mapping.dstTokenAddress, mapping.dstChainId);
          const srcNetwork = await db.getNetwork(mapping.srcChainId);
          const dstNetwork = await db.getNetwork(mapping.dstChainId);

          return {
            ...mapping,
            srcToken,
            dstToken,
            srcNetwork: srcNetwork?.name,
            dstNetwork: dstNetwork?.name
          };
        })
      );

      res.json(enrichedMappings);
    } catch (error) {
      logger.error("Error fetching mappings:", error);
      res.status(500).json({ error: "Failed to fetch mappings" });
    }
  });

  router.get("/token/:address/:chainId", async (req, res) => {
    try {
      const { address, chainId } = req.params;
      const mappings = await db.getTokenMappings(Number(chainId));
      
      const filtered = mappings.filter(
        m => m.srcTokenAddress.toLowerCase() === address.toLowerCase()
      );

      const enriched = await Promise.all(
        filtered.map(async (mapping) => {
          const dstToken = await db.getToken(mapping.dstTokenAddress, mapping.dstChainId);
          const dstNetwork = await db.getNetwork(mapping.dstChainId);

          return {
            ...mapping,
            dstToken,
            dstNetwork: dstNetwork?.name
          };
        })
      );

      res.json(enriched);
    } catch (error) {
      logger.error("Error fetching token mappings:", error);
      res.status(500).json({ error: "Failed to fetch token mappings" });
    }
  });

  return router;
}