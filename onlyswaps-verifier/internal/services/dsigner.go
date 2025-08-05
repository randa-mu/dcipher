package services

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"net/http"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/types"
)

var ErrInvalidDST = errors.New("invalid DST")

var dsignDST string = "dsigner-v01-BN254G1_XMD:KECCAK-256_SVDW_RO_"

type DsignerClient struct {
	host   string
	client *http.Client
}

func NewDsignerClient(cfg *config.Config) *DsignerClient {
	return &DsignerClient{
		host: cfg.DsignerHost,
		client: &http.Client{
			Timeout: cfg.DsignerTimeout,
		},
	}
}

func (d *DsignerClient) Sign(ctx context.Context, request types.ValidationRequest) ([]byte, error) {
	jsonData, err := json.Marshal(request)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal request: %w", err)
	}

	url := fmt.Sprintf("%s/sign", d.host)
	req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, fmt.Errorf("failed to create dsigner request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")

	resp, err := d.client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("failed to make dsigner request: %w", err)
	}
	defer func() {
		if err := resp.Body.Close(); err != nil {
			fmt.Printf("failed to close response body: %v\n", err)
		}
	}()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("dsigner returned status %d", resp.StatusCode)
	}

	var response types.DsignerResponse
	if err := json.NewDecoder(resp.Body).Decode(&response); err != nil {
		return nil, fmt.Errorf("failed to decode response: %w", err)
	}
	if response.DST != dsignDST {
		slog.Error("invalid dsign DST", "got", response.DST)
		return nil, errors.Join(err, ErrInvalidDST)
	}

	return response.Signature, nil
}
