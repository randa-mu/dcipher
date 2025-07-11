# omnievent - a multi chain event listener

## gRPC

We support various operations through gRPC: register events, obtain a stream of upcoming event occurrences, and fetch historical event occurrences.

One easy way to interact with the server with a cli is to rely on [`grpcurl`](https://github.com/fullstorydev/grpcurl).

### Register a new event
To register an event, you need to provide a chain id, a contract address, a requested block safety level, the name of the event and its fields.

This can be done with the following command:
```bash
> grpcurl -import-path ./dcipher-proto -proto omnievent/events.proto -plaintext -d '{"chain_id": 1337, "address": "IO7wOMg7eg81fUq8ZLj2OUJ9evY=", "event_name": "StringEmitted", "fields": [{"sol_type": "string", "indexed": false}], "block_safety": "BLOCK_SAFETY_LATEST" }' 127.0.0.1:8080 events.OmniEventService/RegisterEvent
{
  "uuid": "kfHmBD/yWd+xLKNn4mq41w=="
}
```

It returns a deterministic `uuid` v5 which is obtained from the protobuf encoding of the registration request.

### Stream event occurrences
Upcoming event occurrences can be streamed as followed by specifying the event identifier:
```bash
> grpcurl -import-path ./dcipher-proto -proto omnievent/events.proto -plaintext -d '{"event_uuids": ["kfHmBD/yWd+xLKNn4mq41w=="]}' 127.0.0.1:8080 events.OmniEventService/StreamEvents                         
{
  "eventUuid": "kfHmBD/yWd+xLKNn4mq41w==",
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
> grpcurl -import-path ./dcipher-proto -proto omnievent/events.proto -plaintext -d '{"event_uuids": ["kfHmBD/yWd+xLKNn4mq41w=="], "filter": {"data_filters": [{"data_index": 0, "string": {"exact_values": ["Hello World!"]}}]}}' 127.0.0.1:8080 events.OmniEventService/GetHistoricalEvents
{
  "occurrences": [
    {
      "eventUuid": "kfHmBD/yWd+xLKNn4mq41w==",
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


### Obtain latest event occurrence
It is also possible to fetch the latest occurrence of an event on any chain with the following endpoint:
```bash
> grpcurl -import-path ./dcipher-proto -proto omnievent/events.proto -plaintext -d '{"event_uuids": ["kfHmBD/yWd+xLKNn4mq41w=="], "filter": {"data_filters": [{"data_index": 0, "string": {"exact_values": ["Hello World!"]}}]}}' 127.0.0.1:8080 events.OmniEventService/GetLatestOccurrence
{
  "eventUuid": "kfHmBD/yWd+xLKNn4mq41w==",
  "blockInfo": {
    "blockNumber": "3",
    "blockHash": "dXqsXckoHH/Vic/uR+9lO7xIoehCnI8LMkhBiLM+620=",
    "timestamp": "2025-07-11T15:18:45Z"
  },
  "rawLogData": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADEhlbGxvIFdvcmxkIQAAAAAAAAAAAAAAAAAAAAAAAAAA",
  "eventData": [
    {
      "solType": "string",
      "stringValue": "Hello World!"
    }
  ],
  "chainId": "1337",
  "address": "IO7wOMg7eg81fUq8ZLj2OUJ9evY="
}
```

## Compilation
Due to a compile-time verification of SQL queries, you will need to specify a database url as follows when compiling:  
`DATABASE_URL=sqlite:///tmp/temp.db cargo build --all-features --all-targets`

### Execute omnievent
A test server can be started by running:
```bash
> cargo run -p omnievent --example test-server --features=sqlite -- -p 8080 --database sqlite::memory:
2025-07-11T14:19:39.744279Z  INFO test_server: anvil 1337: http://localhost:1337
2025-07-11T14:19:39.744344Z  INFO test_server: anvil 1338: http://localhost:1338
2025-07-11T14:19:39.744351Z  INFO test_server: funded wallet: 0x836fd4eecd5fc23eb480581cf91f638b5dacfa6ffa3a931b1f0421a5d58cfa5a
2025-07-11T14:19:39.900032Z  INFO test_server: emitter contract deployed at 0x20EEF038C83B7a0f357D4aBC64b8f639427D7Af6
2025-07-11T14:19:39.901266Z  INFO test_server: omnievent service listening on 127.0.0.1:8080
``` 
This requires anvil to be installed and in path.

With the above command, the server starts on port 8080 and with an in-memory sqlite database.
It also starts two anvil chains, with ids 1337 and 1338, respectively.

The test server also deploys the following emitter contract:
```solidity
contract EventEmitter {
    event StringEmitted(string value);
    event Subscribed(address indexed subscriber, uint256 indexed subId);

    function emitString(string calldata _value) external {
        emit StringEmitted(_value);
    }

    function emitSubscribed(uint256 calldata _sub_id) external {
        emit Subscribed(msg.sender, _sub_id);
    }
}
```

You can easily emit events with the following cast command:
```bash
> cast send 0x20EEF038C83B7a0f357D4aBC64b8f639427D7Af6 "emitString(string)()" 'Hello World!' --private-key 0x836fd4eecd5fc23eb480581cf91f638b5dacfa6ffa3a931b1f0421a5d58cfa5a --rpc-url http://localhost:1337 
```
