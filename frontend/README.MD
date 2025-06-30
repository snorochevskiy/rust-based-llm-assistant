# Frontend

## Running locally

Make sure you have trunk and wasm32-unknown-unknown target installed:

```shell
cargo install trunk
rustup target add wasm32-unknown-unknown
```

To run local development frontend server:

```shell
trunk serve --port=3000 --open
```
