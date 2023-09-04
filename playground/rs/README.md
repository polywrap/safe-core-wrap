## Rust Safe Core Wrap Demo

1. Change directory to `rs`:

```
cd rs
```

2. You must have rust installed on your machine. If you don't, you can install it [here](https://www.rust-lang.org/tools/install). Make sure you're using stable version and not nightly

3. You can update the salt nonce in the `src/lib.rs` file. This salt nonce is used later to create the new Safe address.

```rs
const SALT_NONCE: &str = "0x185593";
```

4. Now you should be able to run any available script from Rust! You can just run `cargo run --bin script_name --release` from terminal. You can see available scripts [here](../README.md#available-scripts)

