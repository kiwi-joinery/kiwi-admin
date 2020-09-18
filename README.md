
- `cargo install wasm-pack`
- `cargo +nightly install miniserve`
- `npm install --global rollup`


- `wasm-pack build --target web`
- `rollup ./main.js --format iife --file ./pkg/bundle.js`
- `miniserve . --index index.html -p 8001`
