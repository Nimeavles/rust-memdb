# Rust MemDb

A basic memdb just for fun in order to continue learning rust 🦀.

## Connecting

```bash
cd server
cargo run --release #The server starts at port 4000

nc localhost 4000
```

## Usage

```txt
SET MY_CAR FERRARI

GET MY_CAR -> FERRARI

DEL MY_CAR
```

<small>Made by @Nimeavles</small>
