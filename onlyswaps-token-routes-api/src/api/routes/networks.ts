import { Router } from "express";
import { DatabaseService } from "../../services/database";
import { logger } from "../../utils/logger";

export function createNetworksRouter(db: DatabaseService) {
  const router = Router();

  router.get("/", async (req, res) => {
    try {
      const networks = await db.getNetworks();
      res.json(networks);
    } catch (error) {
      logger.error("Error fetching networks:", error);
      res.status(500).json({ error: "Failed to fetch networks" });
    }
  });

  router.get("/:chainId", async (req, res) => {
    try {
      const chainId = Number(req.params.chainId);
      const network = await db.getNetwork(chainId);
      
      if (!network) {
        return res.status(404).json({ error: "Network not found" });
      }
      
      res.json(network);
    } catch (error) {
      logger.error("Error fetching network:", error);
      res.status(500).json({ error: "Failed to fetch network" });
    }
  });

  return router;
}