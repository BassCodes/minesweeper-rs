# Rust Minesweeper



# Running
To run the desktop version:
```
cargo run
```

To compile to WASM:
```
cargo build --target wasm32-unknown-unknown --release
```
To run that WASM, copy the resulting `target/wasm32-unknown-unknown/release/wasm` into the same directory as `minesweeper.html` and serve both files through a webserver.