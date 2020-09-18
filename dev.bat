call wasm-pack.exe build --target web -- --features console_error_panic_hook &&^
call rollup ./main.js --format iife --file ./pkg/bundle.js &&^
call miniserve.exe . --index index.html -p 8001
