import { Low } from "lowdb";
import { JSONFile } from "lowdb/node";
import path from "path";
import { Database, TokenMapping, TokenInfo, NetworkState } from "../types";

export class DatabaseService {
  private db!: Low<Database>;

  async initialize() {
    const file = path.join(__dirname, "../../data/db.json");
    const adapter = new JSONFile<Database>(file);
    this.db = new Low(adapter, {
      networks: [],
      tokens: [],
      mappings: []
    });

    await this.db.read();
    
    // Initialize with default data if empty
    if (!this.db.data.networks.length) {
      this.db.data.networks = [];
      this.db.data.tokens = [];
      this.db.data.mappings = [];
      await this.db.write();
    }
  }

  async addTokenMapping(mapping: Omit<TokenMapping, "id" | "timestamp">) {
    await this.db.read();
    
    const id = `${mapping.srcTokenAddress}-${mapping.srcChainId}-${mapping.dstTokenAddress}-${mapping.dstChainId}`;
    const existingIndex = this.db.data.mappings.findIndex(m => m.id === id);

    const newMapping: TokenMapping = {
      ...mapping,
      id,
      timestamp: Date.now()
    };

    if (existingIndex >= 0) {
      this.db.data.mappings[existingIndex] = {
        ...this.db.data.mappings[existingIndex],
        isActive: true,
        blockNumber: mapping.blockNumber,
        txHash: mapping.txHash,
        timestamp: Date.now()
      };
    } else {
      this.db.data.mappings.push(newMapping);
    }

    await this.db.write();
  }

  async removeTokenMapping(params: {
    srcTokenAddress: string;
    srcChainId: number;
    dstTokenAddress: string;
    dstChainId: number;
  }) {
    await this.db.read();
    
    const id = `${params.srcTokenAddress}-${params.srcChainId}-${params.dstTokenAddress}-${params.dstChainId}`;
    const mapping = this.db.data.mappings.find(m => m.id === id);

    if (mapping) {
      mapping.isActive = false;
      mapping.timestamp = Date.now();
      await this.db.write();
    }
  }

  async upsertToken(token: TokenInfo) {
    await this.db.read();
    
    const existingIndex = this.db.data.tokens.findIndex(
      t => t.address.toLowerCase() === token.address.toLowerCase() && t.chainId === token.chainId
    );

    if (existingIndex >= 0) {
      this.db.data.tokens[existingIndex] = {
        ...this.db.data.tokens[existingIndex],
        ...token
      };
    } else {
      this.db.data.tokens.push(token);
    }

    await this.db.write();
  }

  async updateNetworkBlockNumber(chainId: number, blockNumber: number) {
    await this.db.read();
    
    const network = this.db.data.networks.find(n => n.chainId === chainId);
    if (network) {
      network.blockNumber = blockNumber;
      await this.db.write();
    }
  }

  async upsertNetwork(network: NetworkState) {
    await this.db.read();
    
    const existingIndex = this.db.data.networks.findIndex(n => n.chainId === network.chainId);
    
    if (existingIndex >= 0) {
      // Update existing, preserving blockNumber if higher
      const existing = this.db.data.networks[existingIndex];
      this.db.data.networks[existingIndex] = {
        ...network,
        blockNumber: Math.max(existing.blockNumber || 0, network.blockNumber || 0)
      };
    } else {
      this.db.data.networks.push(network);
    }
    
    await this.db.write();
  }

  async getTokenMappings(srcChainId?: number, dstChainId?: number): Promise<TokenMapping[]> {
    await this.db.read();
    
    let mappings = this.db.data.mappings.filter(m => m.isActive);
    
    if (srcChainId !== undefined) {
      mappings = mappings.filter(m => m.srcChainId === srcChainId);
    }
    
    if (dstChainId !== undefined) {
      mappings = mappings.filter(m => m.dstChainId === dstChainId);
    }
    
    return mappings;
  }

  async getToken(address: string, chainId: number): Promise<TokenInfo | undefined> {
    await this.db.read();
    return this.db.data.tokens.find(
      t => t.address.toLowerCase() === address.toLowerCase() && t.chainId === chainId
    );
  }

  async getNetworks(): Promise<NetworkState[]> {
    await this.db.read();
    return this.db.data.networks;
  }

  async getNetwork(chainId: number): Promise<NetworkState | undefined> {
    await this.db.read();
    return this.db.data.networks.find(n => n.chainId === chainId);
  }
}