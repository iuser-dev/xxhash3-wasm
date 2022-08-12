<!-- EDIT /Users/z/iuser/xxhash3-wasm/README.md -->

# xxhash3-wasm

wasm wrapper for twox_hash::xxh3::Hash64

see https://docs.rs/twox-hash/latest/twox_hash/xxh3/struct.Hash128.html

use :
[→ test.coffee](test.coffee)

```coffee
#!/usr/bin/env coffee

> ./pkg > hash hash128

byte = 'test'

console.log hash byte
console.log hash128 byte

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

