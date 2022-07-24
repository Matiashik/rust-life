import * as wasm from 'libwasm';
import { createApp } from 'vue'
import './style.css'

createApp({
    data() {
        return {
            phase: 'size-setup',
            height: 7,
            heightR: Array.from((new Array<number>(7).fill(0)).keys()),
            width: 7,
            widthR: Array.from((new Array<number>(7).fill(0)).keys()),
            field: new Array<number>(49).fill(0),
            univ: wasm.Universe.init(7, 7, new Uint8Array(new Array<number>(49).fill(0))),
            tick_loop: false,
            loop_speed: 200
        }
    },
    methods: {
        chphase(n: number) {
            this.phase = ['size-setup', 'life-setup', 'game'][n];
            if (n == 0)  {
                this.tick_loop = false;
                this.field = new Array<number>(this.height * this.width).fill(0)
            }
            if (n == 2) { 
                this.univ = null;
                this.univ = wasm.Universe.init(this.height, this.width, new Uint8Array(this.field));
            }
        },
        getEl(h: number, w: number): String {
            return this.field[h * this.width + w] == 0 ? '⚙' : '🦀';
        },
        setEl(h: number, w: number) {
            this.field[h * this.width + w] = 1 - this.field[h * this.width + w];
        },
        tick() {
            this.tick_loop = !this.tick_loop;
            (async () => {
                let delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
                while (this.tick_loop) {
                    this.univ.tick();
                    this.field = this.univ.get().split("").map((el: String) => Number(el));
                    await delay(this.loop_speed);
                }
            })();
        }
    },
    watch: {
        height(val, _) {
            val = Number(val);
            this.field = new Array<number>(val * this.width).fill(0);
            this.heightR = Array.from((new Array<number>(val).fill(0)).keys())
        },
        width(val, _) {
            val = Number(val)
            this.field = new Array<number>(this.height * val).fill(0);
            this.widthR = Array.from((new Array<number>(val).fill(0)).keys());
        }
    }
}).mount("#app")