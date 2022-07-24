cd ../libwasm/
wasm-pack build
echo "import * as wasm from './libwasm_bg.js';" > pkg/libwasm.js
echo "export * from './libwasm_bg.js';" >> pkg/libwasm.js
cd ../www/
npm i ../libwasm/pkg