package metrics

import (
	"context"
	"errors"
	"log/slog"
	"net/http"
	"sync"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	// Events metrics
	EventsObserved = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "omniswaps_verifier_events_total",
			Help: "Total number of events reported",
		},
		[]string{"chainID"},
	)

	// Signing request metrics
	SigningRequests = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "omniswaps_verifier_signing_requests_total",
			Help: "Total number of signing requests sent to dsigner",
		},
		[]string{"status"},
	)

	// Request duration metrics
	RequestDuration = prometheus.NewHistogramVec(
		prometheus.HistogramOpts{
			Name:    "omniswaps_verifier_dsigner_request_duration_seconds",
			Help:    "Duration of requests to dsigner in seconds",
			Buckets: prometheus.DefBuckets,
		},
		[]string{"endpoint"},
	)

	SwapValidationRequests = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "omniswaps_verifier_swap_validation_requests_total",
			Help: "Total number of swap validation requests",
		},
		[]string{"status"},
	)
)

func init() {
	prometheus.MustRegister(EventsObserved)
	prometheus.MustRegister(SigningRequests)
	prometheus.MustRegister(RequestDuration)
}

func Serve(ctx context.Context, wg *sync.WaitGroup) {
	defer wg.Done()

	mux := http.NewServeMux()
	mux.Handle("/metrics", promhttp.Handler())

	srv := &http.Server{
		Addr:    ":9999",
		Handler: mux,
	}

	wg.Add(1)
	go func() {
		defer wg.Done()
		<-ctx.Done()
		if err := srv.Shutdown(ctx); err != nil {
			slog.Error("failed to shutdown metrics server", "error", err)
		}
	}()

	// blocks until the server is shutdown
	err := srv.ListenAndServe()
	if !errors.Is(err, http.ErrServerClosed) {
		slog.Error("failed to start metrics server", "error", err)
	}
}
