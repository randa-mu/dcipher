# omnievent - a multi chain event listener

## gRPC

We support various operations through gRPC: register events, obtain a stream of upcoming event occurrences, and fetch historical event occurrences.

One easy way to interact with the server with a cli is to rely on [`grpcurl`](https://github.com/fullstorydev/grpcurl).

### Register a new event
To register an event, you need to provide a chain id, a contract address, a requested block safety level, the name of the event and its fields.

This can be done with the following command:
```bash
> grpcurl -import-path ./proto -proto events.proto -plaintext -d '{"chain_id": 1337, "address": "IO7wOMg7eg81fUq8ZLj2OUJ9evY=", "event_name": "StringEmitted", "fields": [{"sol_type": "string", "indexed": false}], "block_safety": "BLOCK_SAFETY_LATEST" }' 127.0.0.1:8089 events.OmniEventService/RegisterEvent
{
  "uuid": "ijWGFy9LUq+s2fJASjY7VQ=="
}
```

It returns a deterministic `uuid` v5 which is obtained from the protobuf encoding of the registration request.

### Stream event occurrences
Upcoming event occurrences can be streamed as followed by specifying the event identifier:
```bash
> grpcurl -import-path ./proto -proto events.proto -plaintext -d '{"event_uuids": ["ijWGFy9LUq+s2fJASjY7VQ=="]}' 127.0.0.1:8089 events.OmniEventService/StreamEvents                         
{
  "eventUuid": "ijWGFy9LUq+s2fJASjY7VQ==",
  "blockInfo": {
    "blockNumber": "2",
    "blockHash": "Ecr/C4bYj5zC69VO1nEuptsgPEOlJpxQwY3St5nQ5p4=",
    "timestamp": "2025-07-07T16:50:51Z"
  },
  "rawLogData": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADEhlbGxvIFdvcmxkIQAAAAAAAAAAAAAAAAAAAAAAAAAA",
  "eventData": [
    {
      "solType": "string",
      "stringValue": "Hello World!"
    }
  ]
}
```

### Obtain historical event occurrences
To obtain past event occurrences with filtering, the following command may be used:
```bash
> grpcurl -import-path ./proto -proto events.proto -plaintext -d '{"event_uuids": ["ijWGFy9LUq+s2fJASjY7VQ=="], "filter": {"data_filters": [{"data_index": 0, "string": {"exact_values": ["Hello World!"]}}]}}' 127.0.0.1:8089 events.OmniEventService/GetHistoricalEvents
{
  "occurrences": [
    {
      "eventUuid": "ijWGFy9LUq+s2fJASjY7VQ==",
      "blockInfo": {
        "blockNumber": "2",
        "blockHash": "Ecr/C4bYj5zC69VO1nEuptsgPEOlJpxQwY3St5nQ5p4=",
        "timestamp": "2025-07-07T16:50:51Z"
      },
      "rawLogData": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADEhlbGxvIFdvcmxkIQAAAAAAAAAAAAAAAAAAAAAAAAAA",
      "eventData": [
        {
          "solType": "string",
          "stringValue": "Hello World!"
        }
      ]
    }
  ]
}
```

## Compilation
Run the server: TODO

Due to a compile-time verification of SQL queries, you will need to specify a database url as follows when compiling the code:  
`DATABASE_URL=sqlite:///tmp/temp.db cargo build --all-features --all-targets`
