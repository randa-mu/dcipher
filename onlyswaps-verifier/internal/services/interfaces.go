package services

import (
	"context"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/proto/omnievent"
)

// ChainMonitorInterface defines the interface for chain monitor operations
type ChainMonitorInterface interface {
	ListenForBridgeReceipt(ctx context.Context, chainID uint64, contractAddress []byte) (chan *omnievent.EventOccurrence, error)
}

// Ensure ChainMonitorClient implements the interface
var _ ChainMonitorInterface = (*ChainMonitorClient)(nil)
