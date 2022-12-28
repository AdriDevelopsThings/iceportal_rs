# iceportal_rs

Fetch data from the iceportal api in a train

# Install
`cargo add iceportal`

# Using

Look to the documentation to see response structs.

## Status query
Make a request to the `/api1/rs/status` api.
```rust
use iceportal::ICEPortal;

let status_response = ICEPortal::fetch_status();
println!("{:?}", status_response);
```

## BAP (Bestellen am Platz) query
Make a request to the `/bap/api/bap-service-status` api.
```rust
use iceportal::ICEPortal;

let bap_response = ICEPortal::fetch_bap();
println!("{:?}", bap_response);
```

## Trip info query
Make a request to the `/api1/rs/tripInfo/trip` api.
```rust
use iceportal::ICEPortal;

let trip_info_response = ICEPortal::fetch_trip_info();
println!("{:?}", trip_info_response);
```

## Connection query
Make a request to the `/api1/rs/tripInfo/connection/EVA_NUMBER` api.
```rust
use iceportal::ICEPortal;

let eva_nr = "8073368";
let connection_response = ICEPortal::fetch_connection(eva_nr);
println!("{:?}", connection_response);
```