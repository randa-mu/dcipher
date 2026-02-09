# Only Swaps Token Mapping Indexer

API service for tracking cross-chain token mappings from only swaps Router contracts deployed across blockchain networks.

## Features

- Indexes `TokenMappingAdded` and `TokenMappingRemoved` Router contract events across multiple chains
- Fetches and caches token metadata (symbol, name, decimals)
- REST API for querying token mappings
- Real-time event listening for new mappings
- JSON file-based database (LowDB) with automatic sync progress tracking
- Multi-chain support (Ethereum, Arbitrum, Base)

## Key Features

- **Multi-chain event indexing:** Tracks TokenMappingAdded/Removed events across all configured chains (Ethereum, Arbitrum, Avalanche, Base, Binance, Filecoin, Linea, Optimism, Scroll, etc.)
- **Configurable sync start:** Set start block per chain in `.env` (sync from any block, or set to 0 to start from latest)
- **Automatic progress tracking:** Sync progress is saved per chain in `data/db.json` and resumes on restart
- **Real-time event listening:** After historical sync, listens for new events and updates database instantly
- **Soft delete for mappings:** `TokenMappingRemoved` events mark mappings as inactive (`isActive: false`), preserving history
- **Token metadata caching:** Fetches and caches ERC20 metadata (symbol, name, decimals) for all mapped tokens
- **Graceful handling of unconfigured chains:** If a mapping references a chain not in `.env`, mapping is stored but token metadata fetch logs a warning
- **REST API endpoints:** Query mappings, tokens, and networks with rich filtering and metadata enrichment
- **Rate limiting and chunked sync:** Configurable block chunk size and request delay for RPC rate limits
- **No sensitive data in DB:** RPC URLs are only in `.env`, never stored in the database
- **Audit/history support:** All mapping changes (add/remove) are tracked with timestamps and block numbers
- **Easy chain addition:** Add new chains by updating `.env` and `src/config/networks.ts`
- **TypeScript codebase:** Strongly typed for reliability and maintainability


## Quick Start

### Prerequisites

- Node.js 18+
- RPC endpoints for Ethereum, Arbitrum, and Base (e.g., Alchemy, Infura)
- only swaps Router contract addresses for each network

### Installation

```bash
# 1. Install dependencies
npm install

# 2. Configure environment
cp .env.example .env
# Edit .env with your RPC URLs, router addresses, and start blocks

# 3. Run in development mode (syncs + starts server)
npm run dev
```

The server will:
- Initialize the database
- Sync historical events from configured start blocks
- Start the API on `http://localhost:3000`
- Listen for new events in real-time

### Configuration

Edit `.env` with your settings:

```env
# Network RPC URLs (required)
ETHEREUM_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY
ARBITRUM_RPC_URL=https://arb-mainnet.g.alchemy.com/v2/YOUR_API_KEY
BASE_RPC_URL=https://base-mainnet.g.alchemy.com/v2/YOUR_API_KEY

# Router Contract Addresses (required)
ETHEREUM_ROUTER=0x16323707e61d20A39AaE5ab64808e480B91658aB
ARBITRUM_ROUTER=0x16323707e61d20A39AaE5ab64808e480B91658aB
BASE_ROUTER=0x16323707e61d20A39AaE5ab64808e480B91658aB

# Starting Block Numbers (contract deployment blocks)
# Set to a specific block number to sync from that block
# Set to 0 to start from latest block (no historical sync, only new events)
ETHEREUM_START_BLOCK=23822063
ARBITRUM_START_BLOCK=401344812
BASE_START_BLOCK=38314492

# API Configuration
PORT=3000
NODE_ENV=development

# Sync Configuration
SYNC_INTERVAL=60000
# BLOCK_CHUNK_SIZE: Number of blocks to query per request
# Alchemy Free tier: max 10 blocks, Growth tier: max 10000 blocks
# Increase this value if you have a paid RPC plan for faster syncing
BLOCK_CHUNK_SIZE=10
# REQUEST_DELAY_MS: Delay between chunk requests to avoid rate limiting
# Alchemy Free tier: 330 req/sec, recommend 200-500ms delay
# Set to 0 for paid plans with higher limits
REQUEST_DELAY_MS=200
```

**Important Notes:**
- **Start Block Behavior:**
  - Set to a specific block number (e.g., `23822063`) to sync all events from that block forward
  - Set to `0` to skip historical sync and only capture new events from the current latest block
  - Once syncing starts, progress is automatically saved to `data/db.json` and resumes on restart
- **Multi-Chain Support:**
  - The indexer only syncs chains configured in `.env`
  - Token mappings may reference destination chains not configured (e.g., BSC, Avalanche, etc.)
  - When fetching metadata for tokens on unconfigured chains, you will see warnings like `"No provider for chain 56"`
  - Mappings are still stored correctly; only the destination token metadata will be missing
  - To add more chains: add RPC URL, router address, and start block to `.env`, then update `src/config/networks.ts`
- **Rate Limiting:** Free tier RPC providers have strict limits. Use `BLOCK_CHUNK_SIZE=10` and `REQUEST_DELAY_MS=200` for Alchemy free tier.

### Production Deployment

```bash
# Build TypeScript
npm run build

# Start production server
npm start

# Or run one-time sync only (no server)
npm run sync
```

## API Endpoints

### Health Check

**GET** `/health`

Check if the API is running.

```bash
curl http://localhost:3000/health
```

Example output:
```json
{
  "status": "ok",
  "timestamp": 1732636800000
}
```

### Networks

**GET** `/api/networks`

List all supported networks.

```bash
curl http://localhost:3000/api/networks
```

Example output:
```json
{
  "networks": [
    {
      "id": "ethereum",
      "name": "Ethereum",
      "chainId": 1,
      "routerAddress": "0x16323707e61d20A39AaE5ab64808e480B91658aB"
    },
    {
      "id": "arbitrum",
      "name": "Arbitrum",
      "chainId": 42161,
      "routerAddress": "0x16323707e61d20A39AaE5ab64808e480B91658aB"
    }
  ]
}
```

**GET** `/api/networks/:chainId`

Get specific network by chain ID.

```bash
curl http://localhost:3000/api/networks/1
```

Example output:
```json
{
  "id": "ethereum",
  "name": "Ethereum",
  "chainId": 1,
  "routerAddress": "0x16323707e61d20A39AaE5ab64808e480B91658aB"
}
```

### Token Mappings

**GET** `/api/mappings`

Get all token mappings (with enriched token metadata).

```bash
curl http://localhost:3000/api/mappings
```

Example output:
```json
{
  "mappings": [
    {
      "id": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48-1-0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174-137",
      "srcTokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "srcChainId": 1,
      "dstTokenAddress": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
      "dstChainId": 137,
      "isActive": true,
      "blockNumber": 12345678,
      "txHash": "0xabc123...",
      "timestamp": 1732636800000,
      "srcToken": {
        "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "chainId": 1,
        "symbol": "USDC",
        "name": "USD Coin",
        "decimals": 6
      },
      "dstToken": {
        "address": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "chainId": 137,
        "symbol": "USDC",
        "name": "USD Coin (PoS)",
        "decimals": 6
      },
      "srcNetwork": "Ethereum",
      "dstNetwork": "Polygon"
    }
  ],
  "count": 1
}
```

**GET** `/api/mappings?srcChainId={id}&dstChainId={id}`

Filter mappings by source and/or destination chain.

```bash
# Get mappings from Ethereum
curl 'http://localhost:3000/api/mappings?srcChainId=1'

# Get mappings to Arbitrum
curl 'http://localhost:3000/api/mappings?dstChainId=42161'

# Get mappings from Ethereum to Base
curl 'http://localhost:3000/api/mappings?srcChainId=1&dstChainId=8453'
```

Example output:
```json
{
  "mappings": [
    {
      "id": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48-1-0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174-137",
      "srcTokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "srcChainId": 1,
      "dstTokenAddress": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
      "dstChainId": 137,
      "isActive": true,
      "blockNumber": 12345678,
      "txHash": "0xabc123...",
      "timestamp": 1732636800000,
      "srcToken": {
        "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "chainId": 1,
        "symbol": "USDC",
        "name": "USD Coin",
        "decimals": 6
      },
      "dstToken": {
        "address": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "chainId": 137,
        "symbol": "USDC",
        "name": "USD Coin (PoS)",
        "decimals": 6
      },
      "srcNetwork": "Ethereum",
      "dstNetwork": "Polygon"
    }
  ],
  "count": 1
}
```

**GET** `/api/mappings/token/:address/:chainId`

Get all destination mappings for a specific source token.

```bash
curl http://localhost:3000/api/mappings/token/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/1
```

Example output:
```json
{
  "mappings": [
    {
      "srcTokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "srcChainId": 1,
      "dstTokenAddress": "0x...",
      "dstChainId": 42161,
      "isActive": true,
      "dstToken": {
        "symbol": "USDC",
        "name": "USD Coin",
        "decimals": 6
      },
      "dstNetwork": "Arbitrum",
      "dstRpcUrl": "https://arb-mainnet.g.alchemy.com/v2/..."
    }
  ]
}
```

### Tokens

**GET** `/api/tokens/:address/:chainId`

Get token metadata for a specific token on a specific chain.

```bash
curl http://localhost:3000/api/tokens/0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48/1
```

Example output:
```json
{
  "token": {
    "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "chainId": 1,
    "symbol": "USDC",
    "name": "USD Coin",
    "decimals": 6
  },
  "networkName": "Ethereum"
}
```

## How It Works

### Event Indexing

The indexer monitors `TokenMappingAdded` and `TokenMappingRemoved` events from only swaps Router contracts:

```solidity
event TokenMappingAdded(uint256 indexed dstChainId, address indexed dstToken, address indexed srcToken);
event TokenMappingRemoved(uint256 indexed dstChainId, address indexed dstToken, address indexed srcToken);
```

### Sync Strategy

The indexer automatically tracks sync progress per chain:

1. **First Run**: Starts from `{NETWORK}_START_BLOCK` configured in `.env`
2. **Subsequent Runs**: Resumes from last synced block stored in database
3. **Progress Tracking**: Each chain's `blockNumber` updates after successful sync
4. **Incremental Syncing**: Only processes new blocks on restart

To re-sync from a specific block, update the `blockNumber` for a network in `data/db.json`.

### Real-time Updates

After historical sync completes, the indexer:
- Listens for new events on each chain
- Automatically processes new mappings as they occur
- Updates the database in real-time

## Database

Data is stored in `data/db.json` with the following structure:

```json
{
  "networks": [
    {
      "chainId": 1,
      "name": "Ethereum",
      "routerAddress": "0x...",
      "blockNumber": 21234567
    }
  ],
  "tokens": [
    {
      "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "chainId": 1,
      "symbol": "USDC",
      "name": "USD Coin",
      "decimals": 6
    }
  ],
  "mappings": [
    {
      "id": "0xA0b8...3606eB48-1-0x2791...9Aa84174-137",
      "srcTokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "srcChainId": 1,
      "dstTokenAddress": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
      "dstChainId": 137,
      "isActive": true,
      "blockNumber": 12345678,
      "txHash": "0xabc123...",
      "timestamp": 1732636800000
    }
  ]
}
```

### Database Schema

- **`networks[]`**: Network sync state (chainId, name, routerAddress, last synced blockNumber)
  - Note: RPC URLs are NOT stored in the database - they come from environment variables only
- **`tokens[]`**: Token metadata cache (symbol, name, decimals)
- **`mappings[]`**: Token mapping records with `isActive` flag for removals

## Monitoring

### Check Sync Progress

View current sync state:

```bash
cat data/db.json | jq '.networks[] | {name, chainId, blockNumber}'
```

### Application Logs

The indexer logs all operations:

```
[INFO] [2025-11-26T12:00:00.000Z] Starting OnlySwaps Indexer...
[INFO] [2025-11-26T12:00:01.000Z] ✓ Database initialized
[INFO] [2025-11-26T12:00:02.000Z] [Chain 1] Syncing from block 21234567 to 21234600
[INFO] [2025-11-26T12:00:05.000Z] [Chain 1] Sync complete. Last block: 21234600
[INFO] [2025-11-26T12:00:05.500Z] [Chain 1] Listening for events...
[INFO] [2025-11-26T12:00:06.000Z] ✓ API server running on http://localhost:3000
```

## Troubleshooting

### RPC Rate Limiting

If syncing is slow or timing out:
- Reduce `BLOCK_CHUNK_SIZE` in `.env` (e.g., from 10000 to 5000)
- Use premium RPC endpoints with higher rate limits
- Set start blocks closer to deployment to reduce sync time

### Events Not Appearing

1. Verify router addresses are correct in `.env`
2. Check start blocks are before first mapping event
3. Ensure RPC URLs are working and have correct API keys
4. Review logs for sync errors

### Port Already in Use

Change the port in `.env`:

```env
PORT=3001
```

## Development

### Project Structure

```
onlyswaps-token-routes-api/
├── src/
│   ├── config/
│   │   └── networks.ts          # Network configurations
│   ├── services/
│   │   ├── database.ts          # LowDB service
│   │   ├── event-indexer.ts     # Blockchain event indexing
│   │   └── token-metadata.ts    # ERC20 metadata fetching
│   ├── api/
│   │   ├── server.ts            # Express server
│   │   └── routes/
│   │       ├── networks.ts      # Network endpoints
│   │       ├── mappings.ts      # Mapping endpoints
│   │       └── tokens.ts        # Token endpoints
│   ├── types/
│   │   └── index.ts             # TypeScript types
│   ├── utils/
│   │   └── logger.ts            # Logging utility
│   ├── index.ts                 # Main entry point
│   └── sync.ts                  # Sync-only script
├── data/
│   └── db.json                  # Database file
└── package.json
```

### Type Checking

```bash
npx tsc --noEmit
```

## License

See parent repository LICENSE file.