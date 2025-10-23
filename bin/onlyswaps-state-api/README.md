# onlyswaps-state-api

A service for listening to onlyswaps events, storing them in a local database, and serving filterable views of the
ingested data over HTTP.

## Endpoint

```
GET /transactions
```

### Description

Retrieves a filtered list of swap transactions stored in the application's current state.

---

### Query Parameters

| Parameter              | Type                | Description                                                                       |
|------------------------|---------------------|-----------------------------------------------------------------------------------|
| `request_id`           | 0x-prefixed hex     | Filters transactions matching a specific request ID.                              |
| `chain_id`             | integer             | Filters transactions where either `src_chain_id` or `dest_chain_id` matches.      |
| `address`              | 0x-prefixed address | Filters transactions where the sender, recipient, or solver matches this address. |
| `sender`               | 0x-prefixed address | Filters transactions by sender address only.                                      |
| `recipient`            | 0x-prefixed address | Filters transactions by recipient address only.                                   |
| `solver`               | 0x-prefixed address | Filters transactions by solver address only.                                      |
| `requested_time_start` | integer             | Inclusive start bound for filtering the `requested_time` in epoch seconds         |
| `requested_time_end`   | integer             | Inclusive end bound for filtering the `requested_time` in epoch seconds           |
| `verified_time_start`  | integer             | Inclusive start bound for filtering the `verified_time` in epoch seconds          |
| `verified_time_end`    | integer             | Inclusive end bound for filtering the `verified_time` in epoch seconds            |
| `limit`                | integer             | Maximum number of transactions to return. Defaults to `100`.                      |
| `offset`               | integer             | Starting offset for pagination. Defaults to `0`.                                  |

---

### Example Request

```
GET /transactions?sender=0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa&limit=2&offset=1
```

---

### Example Response

```json
[
  {
    "request_id": "0x882dc902c46c5aae1dc187e6a22764b64b36fcb79b10d52d079655d63dfef056",
    "src_chain_id": 84532,
    "dest_chain_id": 43113,
    "sender": "0x23bcb0d1706d2733eb0f7f0e757f76957135448a",
    "recipient": "0x23bcb0d1706d2733eb0f7f0e757f76957135448a",
    "token_in": "0x1b0f6cf6f3185872a581bd2b5a738eb52ccd4d76",
    "token_out": "0x1b0f6cf6f3185872a581bd2b5a738eb52ccd4d76",
    "amount_in": "1045025125628140703",
    "amount_out": "1000000000000000000",
    "verification_fee": "5025125628140703",
    "solver_fee": "40000000000000000",
    "state": "verified",
    "solver": "0xebf1b841eff6d50d87d4022372bc1191e781ab68",
    "requested_time": 1761156062,
    "solved_time": 1761156067,
    "verified_time": 1761156070
  }
]
```

---

### Filtering Behavior

#### 1. `request_id`

Filters the vector to include only the transaction with a matching `request_id`.

#### 2. `chain_id`

Matches transactions where either `src_chain_id` or `dest_chain_id` equals the given value.

#### 3. `address`

Matches any transaction where the `sender`, `recipient`, or `solver` equals the provided address.

#### 4. `sender` / `recipient` / `solver`

Each applies a strict equality filter for the specified field.

#### 5. `requested_time_start` / `requested_time_end` / `verified_time_start` / `verified_time_end`

Inclusive integer bounds for epoch seconds.

#### 6. `limit` and `offset`

Pagination is applied after filtering:

- Transactions are skipped by `offset`
- Then truncated to `limit`
- If `offset` exceeds the list length, an empty array is returned

---

### Example Pagination

#### Request

```
GET /transactions?limit=2&offset=1
```

#### Response

```json
[
  {
    "request_id": "...second transaction..."
  },
  {
    "request_id": "...third transaction..."
  }
]
```

If the offset exceeds the available range:

```
GET /transactions?limit=2&offset=100
```

Response:

```json
[]
```
