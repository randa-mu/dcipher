package server

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"sync"

	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/config"
	"github.com/randa-mu/dcipher/onlyswaps-verifier/internal/handlers"
)

type Server struct {
	server *http.Server
}

func NewServer(cfg *config.Config) (*Server, error) {
	mux := http.NewServeMux()

	validationHandler, err := handlers.NewValidationHandler(cfg)
	if err != nil {
		return nil, err
	}

	mux.HandleFunc("GET /v1/swap/latest", validationHandler.GetLatestReceipts)
	mux.HandleFunc("POST /v1/swap/validation", validationHandler.HandleValidationRequest)
	mux.HandleFunc("GET /health", validationHandler.HealthCheck)

	addr := fmt.Sprintf("localhost:%s", cfg.Port)
	srv := &http.Server{
		Addr:    addr,
		Handler: mux,
	}

	return &Server{
		server: srv,
	}, nil
}

func (s *Server) Start(ctx context.Context, wg *sync.WaitGroup) {
	defer wg.Done()

	slog.Info("starting omniswap-verifier server", "address", s.server.Addr)

	wg.Add(1)
	go func() {
		defer wg.Done()
		<-ctx.Done()
		slog.Info("shutting down omniswap-verifier server")
		if err := s.server.Shutdown(context.Background()); err != nil {
			slog.Error("failed to shutdown server gracefully", "error", err)
		}
	}()

	if err := s.server.ListenAndServe(); !errors.Is(err, http.ErrServerClosed) {
		slog.Error("failed to start server", "error", err)
	}
}
