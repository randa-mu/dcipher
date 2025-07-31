package proto

// This file is used to generate the proto files for the project with `go generate ./...`, and nothing else.

//go:generate protoc -I=../../../dcipher-proto --go_out=. --go_opt=paths=source_relative --go_opt=Momnievent/events.proto=github.com/randa-mu/dcipher/onlyswaps-verifier/internal/proto/omnievent --go-grpc_out=. --go-grpc_opt=paths=source_relative --go-grpc_opt=Momnievent/events.proto=github.com/randa-mu/dcipher/onlyswaps-verifier/internal/proto/omnievent omnievent/events.proto
