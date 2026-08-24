#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/pi-agent-space-invaders.wasm web/space_invaders.wasm

if [ ! -f web/mq_js_bundle.js ]; then
  bundle=$(find "${CARGO_HOME:-$HOME/.cargo}/registry/src" -path '*/macroquad-*/js/mq_js_bundle.js' | head -n 1 || true)
  if [ -n "$bundle" ]; then
    cp "$bundle" web/mq_js_bundle.js
  else
    curl -L https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js -o web/mq_js_bundle.js
  fi
fi

echo "Built web/space_invaders.wasm"
echo "Serve with: python3 -m http.server 8000 --directory web"
echo "Then open: http://localhost:8000"
