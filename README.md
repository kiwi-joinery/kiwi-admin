
- `cargo install wasm-pack`
- `npm install --global http-server`
- `npm install --global rollup`


- `wasm-pack build --target web`
- `rollup ./main.js --format iife --file ./pkg/bundle.js`
- `http-server -p 8001 --proxy http://localhost:8001?`
