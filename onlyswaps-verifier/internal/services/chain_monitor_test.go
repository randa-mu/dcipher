package services

import (
	"context"
	"testing"
	"time"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
)

func TestChainMonitorClient_NetworkError(t *testing.T) {
	// Test network error scenario
	cfg := &config.Config{
		ChainMonitorURL:     "http://nonexistent-server:9999",
		ChainMonitorTimeout: 100 * time.Millisecond,
	}
	client, err := NewChainMonitorClient(cfg)
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}

	result, err := client.ListenForBridgeReceipt(context.Background(), 1, []byte{0x00})

	if err == nil {
		t.Error("expected network error but got none")
	}

	if result != nil {
		t.Errorf("expected result false, got %v", result)
	}
}
