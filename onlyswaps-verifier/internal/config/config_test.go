package config

import (
	"os"
	"testing"
	"time"
)

func TestLoadFromEnv_ValidURLs(t *testing.T) {
	tests := []struct {
		name            string
		chainMonitorURL string
		dsignerHost     string
		expectError     bool
	}{
		{
			name:            "valid http URLs",
			chainMonitorURL: "http://localhost:8081",
			dsignerHost:     "http://localhost:8082",
			expectError:     false,
		},
		{
			name:            "valid https URLs",
			chainMonitorURL: "https://api.example.com",
			dsignerHost:     "https://dsigner.example.com",
			expectError:     false,
		},
		{
			name:            "URLs with ports",
			chainMonitorURL: "http://monitor.example.com:9090",
			dsignerHost:     "https://dsigner.example.com:8443",
			expectError:     false,
		},
		{
			name:            "URLs with paths",
			chainMonitorURL: "http://api.example.com/v1/monitor",
			dsignerHost:     "https://api.example.com/dsigner",
			expectError:     false,
		},
		{
			name:            "URLs with trailing slashes - should be normalized",
			chainMonitorURL: "http://localhost:8081/",
			dsignerHost:     "http://localhost:8082/",
			expectError:     false,
		},
		{
			name:            "invalid scheme - ftp",
			chainMonitorURL: "ftp://invalid.com",
			dsignerHost:     "http://localhost:8082",
			expectError:     true,
		},
		{
			name:            "invalid scheme - dsigner",
			chainMonitorURL: "http://localhost:8081",
			dsignerHost:     "ftp://invalid.com",
			expectError:     true,
		},
		{
			name:            "missing scheme",
			chainMonitorURL: "localhost:8081",
			dsignerHost:     "http://localhost:8082",
			expectError:     true,
		},
		{
			name:            "empty host",
			chainMonitorURL: "http://",
			dsignerHost:     "http://localhost:8082",
			expectError:     true,
		},
		{
			name:            "malformed URL",
			chainMonitorURL: "http://localhost:8081",
			dsignerHost:     "://invalid",
			expectError:     true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Set environment variables
			t.Setenv("CHAIN_MONITOR_URL", tt.chainMonitorURL)
			t.Setenv("DSIGNER_HOST", tt.dsignerHost)

			cfg, err := LoadFromEnv()

			if tt.expectError {
				if err == nil {
					t.Error("expected error but got none")
				}
				if cfg != nil {
					t.Error("expected config to be nil when error occurs")
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
				if cfg == nil {
					t.Error("expected config to be non-nil")
				}
				if cfg != nil {
					// For URLs with trailing slashes, expect them to be normalized
					expectedChainURL := tt.chainMonitorURL
					expectedDsignerURL := tt.dsignerHost
					if tt.name == "URLs with trailing slashes - should be normalized" {
						expectedChainURL = "http://localhost:8081"
						expectedDsignerURL = "http://localhost:8082"
					}

					if cfg.ChainMonitorURL != expectedChainURL {
						t.Errorf("expected ChainMonitorURL %s, got %s", expectedChainURL, cfg.ChainMonitorURL)
					}
					if cfg.DsignerHost != expectedDsignerURL {
						t.Errorf("expected DsignerHost %s, got %s", expectedDsignerURL, cfg.DsignerHost)
					}
				}
			}
		})
	}
}

func TestLoadFromEnv_Defaults(t *testing.T) {
	// Clear environment variables
	envVars := []string{
		"PORT", "CHAIN_MONITOR_URL", "DSIGNER_HOST", "CHAIN_MONITOR_TIMEOUT_SECONDS", "DSIGNER_TIMEOUT_SECONDS",
	}

	for _, envVar := range envVars {
		if err := os.Unsetenv(envVar); err != nil {
			t.Logf("failed to unset %s: %v", envVar, err)
		}
	}

	cfg, err := LoadFromEnv()
	if err != nil {
		t.Fatalf("unexpected error with default values: %v", err)
	}

	expectedDefaults := map[string]interface{}{
		"Port":                "8080",
		"ChainMonitorURL":     "http://localhost:8081",
		"DsignerHost":         "http://localhost:8082",
		"ChainMonitorTimeout": 10 * time.Second,
		"DsignerTimeout":      15 * time.Second,
	}

	if cfg.Port != expectedDefaults["Port"] {
		t.Errorf("expected Port %v, got %v", expectedDefaults["Port"], cfg.Port)
	}
	if cfg.ChainMonitorURL != expectedDefaults["ChainMonitorURL"] {
		t.Errorf("expected ChainMonitorURL %v, got %v", expectedDefaults["ChainMonitorURL"], cfg.ChainMonitorURL)
	}
	if cfg.DsignerHost != expectedDefaults["DsignerHost"] {
		t.Errorf("expected DsignerHost %v, got %v", expectedDefaults["DsignerHost"], cfg.DsignerHost)
	}
	if cfg.ChainMonitorTimeout != expectedDefaults["ChainMonitorTimeout"] {
		t.Errorf("expected ChainMonitorTimeout %v, got %v", expectedDefaults["ChainMonitorTimeout"], cfg.ChainMonitorTimeout)
	}
	if cfg.DsignerTimeout != expectedDefaults["DsignerTimeout"] {
		t.Errorf("expected DsignerTimeout %v, got %v", expectedDefaults["DsignerTimeout"], cfg.DsignerTimeout)
	}
}

func TestValidateAndNormalizeHTTPURL(t *testing.T) {
	tests := []struct {
		name        string
		url         string
		expectedURL string
		expectError bool
	}{
		{
			name:        "URL without trailing slash",
			url:         "http://localhost:8080",
			expectedURL: "http://localhost:8080",
			expectError: false,
		},
		{
			name:        "URL with trailing slash - should be normalized",
			url:         "http://localhost:8080/",
			expectedURL: "http://localhost:8080",
			expectError: false,
		},
		{
			name:        "URL with path - trailing slash preserved",
			url:         "http://api.example.com/v1/",
			expectedURL: "http://api.example.com/v1/",
			expectError: false,
		},
		{
			name:        "URL with path - no trailing slash",
			url:         "http://api.example.com/v1",
			expectedURL: "http://api.example.com/v1",
			expectError: false,
		},
		{
			name:        "HTTPS URL with trailing slash",
			url:         "https://secure.example.com/",
			expectedURL: "https://secure.example.com",
			expectError: false,
		},
		{
			name:        "invalid scheme",
			url:         "ftp://example.com/",
			expectedURL: "",
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := validateURL(tt.url, "TEST_URL")

			if tt.expectError {
				if err == nil {
					t.Error("expected error but got none")
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
				if result != tt.expectedURL {
					t.Errorf("expected normalized URL %q, got %q", tt.expectedURL, result)
				}
			}
		})
	}
}
