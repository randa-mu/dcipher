import express from "express";
import cors from "cors";
import { DatabaseService } from "../services/database";
import { createNetworksRouter } from "./routes/networks";
import { createMappingsRouter } from "./routes/mappings";
import { createTokensRouter } from "./routes/tokens";

export function createServer(db: DatabaseService) {
  const app = express();

  app.use(cors());
  app.use(express.json());

  // Health check
  app.get("/health", (req, res) => {
    res.json({ status: "ok", timestamp: new Date().toISOString() });
  });

  // Routes
  app.use("/api/networks", createNetworksRouter(db));
  app.use("/api/mappings", createMappingsRouter(db));
  app.use("/api/tokens", createTokensRouter(db));

  return app;
}