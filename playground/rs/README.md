## Rust Safe Core Wrap Demo

1. Change directory to `rs`:

```
cd rs
```

2. Install rust and necessary dependencies:
```
nvm install && nvm use && yarn
```

3. You can update the salt nonce in the `src/utils` file. This salt nonce is used later to create the new Safe address.

```ts
const SALT_NONCE = "0x185593";
```

4. Now you should be able to run any available script from JS! You can just run `yarn ${script_name}` from terminal. You can see available scripts [here](../README.md#available-scripts)

