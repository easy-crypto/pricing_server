# pricing_server

Price server that serve _BTCUSD_ pricing data.

# Usage

```
PORT=<Server port to bind> \
ADDRESS=<Server address> \
DB_USERNAME=<DB username> \
DB_PASSWORD=<DB password> \
DB_HOSTNAME=<DB hostname> \
DB_NAME=<DB name> \
RUST_LOG_STYLE=auto RUST_LOG=pricing_daemon=info,warn,error cargo run
```

# Endpoints

GET /prices?from=<From date>&limit=<maximum number of results to response>

_Example request_

```
curl http://localhost:8000/prices?from=2021-11-27T13:18:00Z&limit=201
```

_Example response_

```
{
    "next": "2021-11-27T14:06:00Z",
    "prices": [
        {
        "_id": "2021-11-27T14:06:00Z",
        "open": 55160,
        "high": 55213.8,
        "low": 55160,
        "close": 55213.8,
        "vwap": 0.26784495,
        "volume": 0.26784495,
        "count": 12
        }
    ]
}
```
