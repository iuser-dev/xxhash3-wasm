# xxhash3-wasm

wasm wrapper for xxhash-rust

see https://docs.rs/xxhash-rust

use :
[→ test.coffee](test.coffee)

```coffee
#!/usr/bin/env coffee

> ./pkg/index.js > hash hash128 hash128Len

byte = 'test'

utf8encoder = new TextEncoder()

console.log hash byte
console.log hash128Len utf8encoder.encode byte
console.log hash128 utf8encoder.encode 'x\n开始'
console.log hash128 'x\n开始'
console.log hash128 'x\n开始'

```


out :
[→ out.txt](out.txt)

```txt
7526326212778983229n
Uint8Array(16) [
   61, 255, 102, 120, 208,
  225, 114, 104, 240,  64,
   10,  40,  37,  68,  68,
  172
]
```

