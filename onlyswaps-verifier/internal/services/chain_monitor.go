package services

import (
	"context"
	"fmt"
	"log"
	"log/slog"
	"strconv"
	"strings"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/metrics"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/proto/omnievent"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

type ChainMonitorClient struct {
	// our config
	config *config.Config
	// our grpc client from our Proto files
	grpcClient omnievent.OmniEventServiceClient
	// we keep conn around to close it when we're done
	conn *grpc.ClientConn
}

func NewChainMonitorClient(cfg *config.Config) (*ChainMonitorClient, error) {
	// Create gRPC connection to the chain monitor service
	url := strings.TrimPrefix(cfg.ChainMonitorURL, "https://")
	url = strings.TrimPrefix(url, "http://")
	conn, err := grpc.NewClient(url, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, fmt.Errorf("failed to connect to chain monitor: %w", err)
	}

	client := omnievent.NewOmniEventServiceClient(conn)

	return &ChainMonitorClient{
		config:     cfg,
		grpcClient: client,
		conn:       conn,
	}, nil
}

func (c *ChainMonitorClient) Close() error {
	return c.conn.Close()
}

func (c *ChainMonitorClient) ListenForBridgeReceipt(ctx context.Context, chainID uint64, contractAddress []byte) (chan *omnievent.EventOccurrence, error) {
	// First, register the event
	registerEventRequest := &omnievent.RegisterNewEventRequest{
		ChainId:   chainID,
		Address:   contractAddress,
		EventName: "BridgeReceipt",
		// Event fields: bytes32 indexed requestId, uint256 indexed srcChainId, address indexed solver, uint256 amountOut
		Fields: []*omnievent.EventField{
			{
				SolType: "bytes32",
				Indexed: true,
			},
			{
				SolType: "uint256",
				Indexed: true,
			},
			{
				SolType: "address",
				Indexed: true,
			},
			{
				SolType: "uint256",
				Indexed: false,
			},
		},
		BlockSafety: omnievent.BlockSafety_BLOCK_SAFETY_LATEST,
	}

	// Register the event
	registerResp, err := c.grpcClient.RegisterEvent(ctx, registerEventRequest)
	if err != nil {
		slog.Error("failed to register event", "error", err, "chain_id", chainID, "contract_address", contractAddress)
		return nil, fmt.Errorf("failed to register event: %w", err)
	}

	// Create stream request with the event UUID
	streamReq := &omnievent.StreamEventsRequest{
		EventUuids: [][]byte{registerResp.Uuid},
	}

	// Start streaming events
	stream, err := c.grpcClient.StreamEvents(ctx, streamReq)
	if err != nil {
		slog.Error("failed to start event stream", "error", err, "chain_id", chainID, "contract_address", contractAddress)
		return nil, fmt.Errorf("failed to start event stream: %w", err)
	}

	// Create a channel to send events, buffered so that we don't block the stream
	// TODO: make this configurable or consider if it should be unbuffered with a separate goroutine to handle incoming events
	eventChan := make(chan *omnievent.EventOccurrence, 100)

	// Start a goroutine to read from the stream and send to channel
	go func() {
		defer close(eventChan)
		chainIdStr := strconv.FormatUint(chainID, 10)
		for {
			event, err := stream.Recv()
			if err != nil {
				log.Printf("Error receiving event: %v", err)
				return
			}

			select {
			case eventChan <- event:
				metrics.EventsObserved.WithLabelValues(chainIdStr).Inc()
				slog.Debug("event sent to channel", "chain_id", chainIdStr)
				// Event sent successfully
			case <-ctx.Done():
				// Context cancelled, stop streaming
				slog.Info("context cancelled, stopping stream")
				return
			}
		}
	}()

	return eventChan, nil
}

func (c *ChainMonitorClient) GetHistoricalEvent(ctx context.Context, EventUuid, requestId []byte) (*omnievent.EventOccurrence, error) {

	events, err := c.grpcClient.GetHistoricalEvents(ctx, &omnievent.GetHistoricalEventsRequest{
		EventUuids: [][]byte{EventUuid},
		Filter: &omnievent.EventOccurrenceFilter{
			DataFilters: []*omnievent.OccurrenceDataFilter{
				{
					DataIndex: 1,
					Filter: &omnievent.OccurrenceDataFilter_Bytes{
						Bytes: &omnievent.BytesDataFilter{
							ExactValues: [][]byte{requestId},
						},
					},
				},
			},
		},
	})
	if err != nil {
		return nil, fmt.Errorf("failed to get historical events: %w", err)
	}

	if len(events.Occurrences) > 0 {
		return events.Occurrences[0], nil
	}

	return nil, fmt.Errorf("no historical events found for EventUuid and requestID")
}
