import { Router } from "express";
import { DatabaseService } from "../../services/database";
import { logger } from "../../utils/logger";

export function createTokensRouter(db: DatabaseService) {
  const router = Router();

  router.get("/:address/:chainId", async (req, res) => {
    try {
      const { address, chainId } = req.params;
      const token = await db.getToken(address, Number(chainId));
      
      if (!token) {
        return res.status(404).json({ error: "Token not found" });
      }

      const network = await db.getNetwork(token.chainId);
      
      res.json({
        ...token,
        networkName: network?.name
      });
    } catch (error) {
      logger.error("Error fetching token:", error);
      res.status(500).json({ error: "Failed to fetch token" });
    }
  });

  return router;
}