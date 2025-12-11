import { ethers } from "ethers";

export enum LogLevel {
  DEBUG = 0,
  INFO = 1,
  WARN = 2,
  ERROR = 3,
}

class Logger {
  private level: LogLevel;
  private prefix: string;

  constructor(prefix: string = "only swaps", level: LogLevel = LogLevel.INFO) {
    this.prefix = prefix;
    this.level = level;
  }

  setLevel(level: LogLevel) {
    this.level = level;
  }

  private shouldLog(level: LogLevel): boolean {
    return level >= this.level;
  }

  private formatMessage(level: string, message: string, context?: any): string {
    const timestamp = new Date().toISOString();
    const contextStr = context ? ` ${JSON.stringify(context)}` : "";
    return `[${timestamp}] [${this.prefix}] [${level}] ${message}${contextStr}`;
  }

  debug(message: string, context?: any) {
    if (this.shouldLog(LogLevel.DEBUG)) {
      console.debug(this.formatMessage("DEBUG", message, context));
    }
  }

  info(message: string, context?: any) {
    if (this.shouldLog(LogLevel.INFO)) {
      console.log(this.formatMessage("INFO", message, context));
    }
  }

  warn(message: string, context?: any) {
    if (this.shouldLog(LogLevel.WARN)) {
      console.warn(this.formatMessage("WARN", message, context));
    }
  }

  error(message: string, error?: Error | any) {
    if (this.shouldLog(LogLevel.ERROR)) {
      const context = error instanceof Error ? { message: error.message, stack: error.stack } : error;
      console.error(this.formatMessage("ERROR", message, context));
    }
  }

  // Specialized logging for blockchain events
  event(eventName: string, chainId: number, data?: any) {
    this.info(`Event: ${eventName} on chain ${chainId}`, data);
  }

  // Log transaction details
  transaction(txHash: string, chainId: number, description: string) {
    this.info(`Transaction: ${description}`, { txHash, chainId });
  }

  // Log block processing
  blockRange(chainId: number, startBlock: number, endBlock: number) {
    this.debug(`Processing blocks ${startBlock} to ${endBlock} on chain ${chainId}`);
  }

  // Log token mapping events
  tokenMapping(action: "added" | "removed", srcToken: string, srcChainId: number, dstToken: string, dstChainId: number) {
    this.info(`Token mapping ${action}`, {
      srcToken,
      srcChainId,
      dstToken,
      dstChainId
    });
  }

  // Log API requests
  apiRequest(method: string, path: string, statusCode?: number) {
    this.info(`API ${method} ${path}`, { statusCode });
  }

  // Log sync progress
  syncProgress(chainId: number, currentBlock: number, targetBlock: number) {
    const progress = ((currentBlock / targetBlock) * 100).toFixed(2);
    this.info(`Sync progress for chain ${chainId}: ${progress}%`, {
      currentBlock,
      targetBlock
    });
  }
}

// Create and export a default logger instance
export const logger = new Logger("only swaps",
  process.env.LOG_LEVEL === "debug" ? LogLevel.DEBUG : LogLevel.INFO
);

// Export a function to create custom loggers for specific components
export function createLogger(prefix: string, level?: LogLevel): Logger {
  return new Logger(prefix, level);
}