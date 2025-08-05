package types

import (
	"encoding/hex"
	"encoding/json"
)

// ValidationRequest represents the incoming request to validate a given swap
type ValidationRequest struct {
	SwapTxHash []byte `json:"swap_tx_hash"`
	SwapID     []byte `json:"swap_id"`
	SwapStatus bool   `json:"swap_status"`
}

// ValidationResponse represents the response with the signature
type ValidationResponse struct {
	Signature string `json:"signature"`
}

// MarshalJSON implements the Marhsaller interface for ValidationRequest so that they marshal into the format expected by dsigner
func (d *ValidationRequest) MarshalJSON() ([]byte, error) {
	// we need to conform to the Dsigner API which signs the content of the field m
	msg := struct {
		M string `json:"m"`
	}{
		M: "$swap_tx_hash:" + hex.EncodeToString(d.SwapTxHash) + "; swap_id:" + hex.EncodeToString(d.SwapID) + "$",
	}

	return json.Marshal(msg)
}

// DsignerResponse represents the response from the dsigner service
type DsignerResponse struct {
	Signature []byte `json:"signature"`
	DST       string `json:"dst"`
}
