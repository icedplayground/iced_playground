# iced_hello_app
 a simple hello world iced window
 
![Imgur](https://imgur.com/jfJLQVV.png)

> i didn't want to make anything more comlicated than a window with text that says "Hello, world!"

---

# Development
running and building

```sh
cargo run
cargo check
cargo test
cargo clean
```

cargo bundle (here for sample)
```sh
# cargo install cargo-bundle
# https://github.com/burtonageo/cargo-bundle
cargo bundle --release
```

cargo build (here for sample)
```sh
# wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown

# aarch64-apple-darwin
rustup target add aarch64-apple-darwin
cargo build --release --locked --target=aarch64-apple-darwin
```


---

copyright 2025 by nonresistant.near