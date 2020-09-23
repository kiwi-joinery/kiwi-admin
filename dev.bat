call wasm-pack.exe build --target web -- --features console_error_panic_hook &&^
call rollup ./main.js --format iife --file ./pkg/bundle.js &&^
call http-server -p 8001 --proxy http://localhost:8001?
