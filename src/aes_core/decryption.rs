use std::vec;

use super::constants::SBOX_INV;
use super::key::{expand_key, add_round_key};
use super::util::{blockify, gf_mul};

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut plaintext = vec![];

    plaintext
}