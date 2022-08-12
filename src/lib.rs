use std::hash::Hasher;

use twox_hash::xxh3::{Hash128, Hash64, HasherExt};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hash128(input: &[u8]) -> Box<[u8]> {
  let mut h = Hash128::default();
  h.write(input);
  Box::from(h.finish_ext().to_le_bytes())
}

#[wasm_bindgen]
pub fn hash(input: &[u8]) -> u64 {
  let mut h = Hash64::default();
  h.write(input);
  h.finish()
}
/*
#[wasm_bindgen]
pub fn encode_bin(input: &[u8]) -> Vec<u8> {
rmw_utf8::encode(input)
}

#[wasm_bindgen]
pub fn encode(input: String) -> Vec<u8> {
rmw_utf8::encode(input.as_bytes())
}

#[wasm_bindgen]
pub fn decode(input: &[u8]) -> String {
rmw_utf8::decode(input)
}
*/
