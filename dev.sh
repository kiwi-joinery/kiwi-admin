if command -v cmd.exe &>/dev/null; then
  WSL="cmd.exe /c"
else
  WSL=
fi

$WSL wasm-pack build --target web -- --features console_error_panic_hook
$WSL rollup ./main.js --format iife --file ./pkg/bundle.js
$WSL http-server -p 8001 --proxy http://localhost:8001?
