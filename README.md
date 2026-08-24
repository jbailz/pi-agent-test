# pi-agent-test

A tiny Space Invaders clone written in Rust with Macroquad.

## Controls

- Move: `A`/`D` or arrow keys
- Shoot: `Space` or up arrow
- Restart after win/loss: `Enter`

## Native macOS/Linux

```sh
cargo run --release
```

With Nix:

```sh
nix develop
cargo run --release
```

## Browser

```sh
./scripts/build-web.sh
python3 -m http.server 8000 --directory web
```

Then open <http://localhost:8000>.

The browser build uses the same Rust source and compiles to `wasm32-unknown-unknown`.
