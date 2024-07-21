# Flight Path API

## Endpoint

### POST /api/calculate

Accepts a JSON array of flights and returns the sorted flight path.

#### Request

```json
[
    ["SFO", "EWR"],
    ["ATL", "EWR"],
    ["SFO", "ATL"]
]
```
#### Response :
```json
[
    "SFO",
    "EWR"
]
```

#### Steps to run the project:
#### Response :
```
cargo build
cargo run
```