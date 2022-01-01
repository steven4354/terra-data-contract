# Compilation

You will need Rust 1.44.1+ ~~with `wasm32-unknown-unknown` target installed.~~

check rust version via 

```
rustc --version
```

install rustup: https://rustup.rs/

add the target via `rustup target add wasm32-unknown-unknown` (source: https://github.com/rustwasm/book/issues/160)

You can run unit tests on this via:

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/wallet_scores.wasm .
ls -l wallet_scores.wasm
sha256sum wallet_scores.wasm
```

# Wallet Scores Contract

This is a simple contract to demonstrate using admin access to smart contracts and storage of data using Buckets in cosmwasm

