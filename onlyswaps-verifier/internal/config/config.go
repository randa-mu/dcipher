package config

import (
	"fmt"
	"net/url"
	"os"
	"strconv"
	"strings"
	"time"
)

type Config struct {
	// Server configuration
	URL  string
	Port string

	// Chain monitoring service
	ChainMonitorURL string

	// Dsigner service
	DsignerHost string

	// ChainIDs to monitor
	ChainIds []string

	// Contract Addresses
	RouterContracts []string

	// Timeouts
	ChainMonitorTimeout time.Duration
	DsignerTimeout      time.Duration
}

func LoadFromEnv() (*Config, error) {
	port := getEnvWithDefault("PORT", "8060")
	chainMonitorURL := getEnvWithDefault("CHAIN_MONITOR_URL", "http://localhost:8080")
	dsignerHost := getEnvWithDefault("DSIGNER_HOST", "http://localhost:8082")

	// Validate and normalize Chain Monitor URL
	chainMonitorURL, err := validateURL(chainMonitorURL, "CHAIN_MONITOR_URL")
	if err != nil {
		return nil, err
	}

	// Validate and normalize Dsigner Host URL
	dsignerHost, err = validateURL(dsignerHost, "DSIGNER_HOST")
	if err != nil {
		return nil, err
	}

	chainMonitorTimeout := 10 * time.Second
	if val := os.Getenv("CHAIN_MONITOR_TIMEOUT_SECONDS"); val != "" {
		if parsed, err := strconv.Atoi(val); err == nil {
			chainMonitorTimeout = time.Duration(parsed) * time.Second
		}
	}

	dsignerTimeout := 15 * time.Second
	if val := os.Getenv("DSIGNER_TIMEOUT_SECONDS"); val != "" {
		if parsed, err := strconv.Atoi(val); err == nil {
			dsignerTimeout = time.Duration(parsed) * time.Second
		}
	}

	return &Config{
		Port:                port,
		ChainMonitorURL:     chainMonitorURL,
		DsignerHost:         dsignerHost,
		ChainMonitorTimeout: chainMonitorTimeout,
		DsignerTimeout:      dsignerTimeout,
		// TODO: remove this once we have a proper way to get the chain ids
		ChainIds: []string{"1337", "1338"},
	}, nil
}

func getEnvWithDefault(key, defaultValue string) string {
	if val := os.Getenv(key); val != "" {
		return val
	}
	return defaultValue
}

func validateURL(urlStr, envVarName string) (string, error) {
	parsedURL, err := url.Parse(urlStr)
	if err != nil {
		return "", fmt.Errorf("invalid URL for %s: %w", envVarName, err)
	}

	if parsedURL.Scheme != "http" && parsedURL.Scheme != "https" {
		return "", fmt.Errorf("invalid scheme for %s: must be http or https, got %s", envVarName, parsedURL.Scheme)
	}

	if parsedURL.Host == "" {
		return "", fmt.Errorf("invalid URL for %s: missing host", envVarName)
	}

	// removing trailing slash
	return strings.TrimSuffix(parsedURL.String(), "/"), nil
}
