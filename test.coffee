#!/usr/bin/env coffee

> ./pkg/index.js > hash hash128 hash128Len

byte = 'test'

utf8encoder = new TextEncoder()

console.log hash byte
console.log hash128Len utf8encoder.encode byte
console.log hash128 utf8encoder.encode 'x\n开始'
console.log hash128 'x\n开始'
console.log hash128 'x\n开始'

