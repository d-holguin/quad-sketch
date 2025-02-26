https://beltoforion.de/en/game_of_life/


##### To build wasm 
```bash
cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/game_of_life.wasm .
```
if installed 
```bash
basic-http-server .
```
