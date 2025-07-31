package main

import (
	"context"
	"fmt"
	"log"
	"log/slog"
	"os"
	"os/signal"
	"sync"
	"syscall"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/metrics"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/server"
)

func main() {

	slog.SetLogLoggerLevel(slog.LevelDebug)

	fmt.Println("OnlySwaps-Verifier welcomes you, do you welcome it?")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Load configuration from environment
	cfg, err := config.LoadFromEnv()
	if err != nil {
		slog.Error("failed to load configuration", "error", err)
		os.Exit(1)
	}

	slog.Info("starting OnlySwaps-Verifier",
		"port", cfg.Port,
		"chain_monitor_url", cfg.ChainMonitorURL,
		"dsigner_host", cfg.DsignerHost,
	)

	wg := sync.WaitGroup{}

	// Start metrics server
	wg.Add(1)
	go metrics.Serve(ctx, &wg)

	// Start main HTTP server
	srv, err := server.NewServer(cfg)
	if err != nil {
		log.Fatalf("Failed to create server: %v", err)
	}
	wg.Add(1)
	go srv.Start(ctx, &wg)

	// Wait for signal to shutdown
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	slog.Info("shutting down OnlySwaps Verifier")
	cancel()

	// Wait for all goroutines to finish
	wg.Wait()
	slog.Info("OnlySwaps verifier shutdown complete")

}
