cargo build --target wasm32-unknown-unknown
cd target\wasm32-unknown-unknown\debug
wasm-bindgen --target web --no-typescript --out-dir . minesweeper.wasm
wasm-gc minesweeper.wasm
cd ..\..\..
xcopy .\target\wasm32-unknown-unknown\debug\minesweeper_bg.wasm .\www /K /D /H /Y
xcopy .\target\wasm32-unknown-unknown\debug\minesweeper.js .\www /K /D /H /Y