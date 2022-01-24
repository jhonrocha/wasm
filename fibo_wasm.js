import { fibo } from './pkg/wasm.js'

console.log(fibo(BigInt(10000)))
