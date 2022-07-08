import { default as init } from './minesweeper.js';

async function load_wasm() {
    try {
        console.log("Loading wasm...");
        await init('./minesweeper_bg.wasm');
        console.log("Wasm was loaded successfully");
    } catch (e) {
        console.error(e);
    }
}

document.addEventListener("DOMContentLoaded", load_wasm);