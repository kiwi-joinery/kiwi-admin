import init, { run_app } from './pkg/kiwi_admin.js';
async function main() {
    await init('/pkg/kiwi_admin_bg.wasm');
    run_app();
}
main()