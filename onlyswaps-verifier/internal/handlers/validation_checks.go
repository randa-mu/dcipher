package handlers

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"net/http"
	"strconv"
	"time"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/metrics"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/proto/omnievent"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/services"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/types"
)

type ValidationHandler struct {
	chainMonitor *services.ChainMonitorClient
	dsigner      *services.DsignerClient
	streams      map[string]chan *omnievent.EventOccurrence
}

func NewValidationHandler(cfg *config.Config) (*ValidationHandler, error) {
	chainMonitor, err := services.NewChainMonitorClient(cfg)
	if err != nil {
		return nil, err
	}

	streams := make(map[string]chan *omnievent.EventOccurrence)

	// TODO: remove this once we have a proper way to get the contract address
	contractAddress, err := hex.DecodeString("20EEF038C83B7a0f357D4aBC64b8f639427D7Af6")
	if err != nil {
		return nil, err
	}

	for _, chainID := range cfg.ChainIds {
		slog.Debug("listening for bridge receipt", "chain_id", chainID)
		chainIDUint, err := strconv.ParseUint(chainID, 10, 64)
		if err != nil {
			slog.Error("failed to parse chain id", "error", err)
			return nil, err
		}
		streams[chainID], err = chainMonitor.ListenForBridgeReceipt(context.Background(), chainIDUint, contractAddress)
		if err != nil {
			slog.Error("failed to listen for bridge receipt", "error", err)
			return nil, err
		}
	}

	return &ValidationHandler{
		chainMonitor: chainMonitor,
		dsigner:      services.NewDsignerClient(cfg),
		streams:      streams,
	}, nil
}

func (h *ValidationHandler) HandleValidationRequest(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() {
		metrics.RequestDuration.WithLabelValues("validate").Observe(time.Since(start).Seconds())
	}()

	// Only allow POST requests
	if r.Method != http.MethodPost {
		metrics.SwapValidationRequests.WithLabelValues("method_not_allowed").Inc()

		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	w.Header().Set("Content-Type", "application/json")

	var req types.ValidationRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		slog.Error("failed to parse request", "error", err)
		metrics.SwapValidationRequests.WithLabelValues("invalid_request").Inc()

		http.Error(w, "Invalid request format", http.StatusBadRequest)
		return
	}
	if err := r.Body.Close(); err != nil {
		slog.Error("failed to close request body", "error", err)
	}

	// Validate required fields
	if len(req.SwapID) == 0 || len(req.SwapTxHash) == 0 {
		slog.Error("missing required fields in request")
		metrics.SwapValidationRequests.WithLabelValues("missing_fields").Inc()

		http.Error(w, "Missing one or more required fields: wallet_address, challenge, subscription_id", http.StatusBadRequest)
		return
	}

	// Check status of swap from chain monitoring service
	swapOk, err := h.ValidateSwap(r.Context(), req.SwapID, req.SwapTxHash)
	if err != nil {
		slog.Error("failed to check swap status", "error", err)
		metrics.SwapValidationRequests.WithLabelValues("swap_status_check_failed").Inc()
		http.Error(w, "Failed to check swap status", http.StatusInternalServerError)
		return
	}

	if swapOk {
		metrics.SwapValidationRequests.WithLabelValues("swap_status_true").Inc()
	} else {
		metrics.SwapValidationRequests.WithLabelValues("swap_status_false").Inc()
	}

	req.SwapStatus = swapOk

	// Forward to dsigner for threshold signature
	sig, err := h.dsigner.Sign(r.Context(), req)
	if err != nil || len(sig) < 32 {
		slog.Error("failed to get signature from dsigner", "error", err)
		metrics.SigningRequests.WithLabelValues("error").Inc()
		metrics.SwapValidationRequests.WithLabelValues("signing_failed").Inc()

		http.Error(w, "Failed to get dsigner sig", http.StatusInternalServerError)
		return
	}

	metrics.SigningRequests.WithLabelValues("success").Inc()

	slog.Info("successfully validated and signed", "swap_id", req.SwapID)
	metrics.SwapValidationRequests.WithLabelValues("success").Inc()
	if err := json.NewEncoder(w).Encode(types.ValidationResponse{
		Signature: hex.EncodeToString(sig),
	}); err != nil {
		slog.Error("failed to encode validation response", "error", err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
	}
}

func (h *ValidationHandler) HealthCheck(w http.ResponseWriter, r *http.Request) {
	slog.Debug("received request to health check")

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(map[string]string{
		"status":  "healthy",
		"service": "omniswaps-verifier",
	}); err != nil {
		slog.Error("failed to encode health check response", "error", err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
	}
}

func (h *ValidationHandler) ValidateSwap(ctx context.Context, swapUUID, requestId []byte) (bool, error) {

	// fetch the event from the chain monitor
	event, err := h.chainMonitor.GetHistoricalEvent(ctx, swapUUID, requestId)
	if err != nil {
		return false, err
	}

	// TODO: check if the event is valid

	//	TransferParams({
	//	           sender: msg.sender,
	//	           recipient: recipient,
	//	           token: token,
	//	           amount: amount,
	//	           srcChainId: thisChainId,
	//	           dstChainId: dstChainId,
	//	           swapFee: swapFeeAmount,
	//	           solverFee: solverFeeAmount,
	//	           nonce: nonce,
	//	           executed: false
	//	       });

	if event.ChainId != 1337 {
		slog.Error("invalid chain id", "chain_id", event.ChainId)
	}
	// TODO: this is mocked for now, we need to check if the event is valid
	return true, nil
}

func (h *ValidationHandler) GetLatestReceipts(w http.ResponseWriter, r *http.Request) {
	slog.Info("received request to get latest events")
	w.Header().Set("Content-Type", "application/json")

	events := make([]*omnievent.EventOccurrence, 0)
	count := 0
	for chainID, stream := range h.streams {
		slog.Debug("checking stream", "chain_id", chainID)
		select {
		case event := <-stream:
			slog.Info("received event", "chain_id", chainID)
			events = append(events, event)
			count++
		default:
			slog.Info("no event received")
		}
	}

	if count == 0 {
		slog.Debug("no events received")
		http.Error(w, "No events received", http.StatusNoContent)
		return
	}

	if err := json.NewEncoder(w).Encode(events); err != nil {
		slog.Error("failed to encode event", "error", err)
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}
}
