# onlyswaps-state-api

A service for listening to onlyswaps events, storing them in a local database, and serving filterable views of the ingested data over HTTP.

## Endpoint

```
GET /transactions
```

### Description

Retrieves a filtered list of swap transactions stored in the application's current state.

---

### Query Parameters

| Parameter | Type | Description |
|------------|------|-------------|
| `request_id` | `string (FixedBytes<32>)` | Filters transactions matching a specific request ID. |
| `chain_id` | `number` | Filters transactions where either `src_chain_id` or `dest_chain_id` matches. |
| `address` | `string (Address)` | Filters transactions where the sender, recipient, or solver matches this address. |
| `sender` | `string (Address)` | Filters transactions by sender address only. |
| `recipient` | `string (Address)` | Filters transactions by recipient address only. |
| `solver` | `string (Address)` | Filters transactions by solver address only. |
| `limit` | `integer` | Maximum number of transactions to return. Defaults to `100`. |
| `offset` | `integer` | Starting offset for pagination. Defaults to `0`. |

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
    "request_id": "0x1111111111111111111111111111111111111111111111111111111111111111",
    "src_chain_id": 1,
    "dest_chain_id": 2,
    "sender": "0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Caaaa",
    "recipient": "0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76C2222",
    "solver": "0x17B3cAb3cD7502C6b85ed2E11Fd5988AF76Ceeee",
    "amount_in": "0x0",
    "amount_out": "0x0",
    "verification_fee": "0x0",
    "solver_fee": "0x0",
    "state": "FULFILLED",
    "requested_time": "0x0",
    "solved_time": "0x0",
    "verified_time": "0x0"
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

#### 5. `limit` and `offset`
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
  { "request_id": "...second transaction..." },
  { "request_id": "...third transaction..." }
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
