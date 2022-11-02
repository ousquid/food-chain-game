# food-chain-game

```
  __                 _      _           _                                    
 / _| ___   ___   __| | ___| |__   __ _(_)_ __     __ _  __ _ _ __ ___   ___ 
| |_ / _ \ / _ \ / _` |/ __| '_ \ / _` | | '_ \   / _` |/ _` | '_ ` _ \ / _ \
|  _| (_) | (_) | (_| | (__| | | | (_| | | | | | | (_| | (_| | | | | | |  __/
|_|  \___/ \___/ \__,_|\___|_| |_|\__,_|_|_| |_|  \__, |\__,_|_| |_| |_|\___|
                                                  |___/                      
```

## build
### native
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo apt-get install libasound2-dev libudev-dev
$ cargo run
```

### web
```
$ cargo build --target wasm32-unknown-unknown --release
$ wasm-bindgen --out-dir ./out --target web --no-typescript ./target/wasm32-unknown-unknown/release/
```

## game play

https://ousquid.github.io/food-chain-game/