if cat /proc/version | grep microsoft; then
  CMD="cmd.exe /c"
  PWD=$(wslpath -w $(pwd))
else
  CMD=
  PWD=$(pwd)
fi

$CMD wasm-pack build --dev --target web -- --features console_error_panic_hook
$CMD rollup ./main.js --format iife --file ./pkg/bundle.js
$CMD docker run -it --rm -p 8001:80 -v $PWD:/usr/share/nginx/html -v $PWD/nginx:/etc/nginx:ro nginx
