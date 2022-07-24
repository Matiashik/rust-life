import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import vue from '@vitejs/plugin-vue';

export default defineConfig({
    server: {
        port: 1488,
        strictPort: true
    },
    plugins: [wasm(), topLevelAwait(), vue()],
    resolve: {
        alias: {
            'vue': 'vue/dist/vue.esm-bundler.js'
        }
    }
})