use std::vec;

use rand::TryRngCore;
use rand::rngs::OsRng;

use super::constants::SBOX;
use super::shared::{add_round_key, expand_key, gf_mul};

// start with ECB chaining
pub fn encrypt(plaintext: &[u8], key: Option<&[u8]>) -> Vec<u8> {
    let rand_key;
    let key = match key {
        Some(k) => k,
        None => {
            rand_key = random_key();
            &rand_key
        }
    };

    let round_keys = expand_key(&key);

    // turn plaintext into Vec<[[u8; 4]; 4]>
    // for block in plaintext { ciphertext += encrypt_block(&block, &round_keys) }

    let mut ciphertext = vec![];
    ciphertext
}

fn encrypt_block(plaintext: &[[u8; 4]; 4], round_keys: &[[[u8; 4]; 4]]) -> [[u8; 4]; 4] {
    let mut state = plaintext.clone();
    let num_rounds = round_keys.len();

    add_round_key(&mut state, &round_keys[0]);

    for round_key in &round_keys[1..num_rounds - 1] {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, round_key);
    }

    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, round_keys.last().unwrap());

    state
}

fn random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng
        .try_fill_bytes(&mut key)
        .expect("key generation failed");
    key
}

fn sub_bytes(state: &mut [[u8; 4]; 4]) {
    for word in state {
        for byte in word {
            *byte = SBOX[*byte as usize];
        }
    }
}

fn shift_rows(state: &mut [[u8; 4]; 4]) {
    let s = *state;
    *state = [
        [s[0][0], s[1][1], s[2][2], s[3][3]],
        [s[1][0], s[2][1], s[3][2], s[0][3]],
        [s[2][0], s[3][1], s[0][2], s[1][3]],
        [s[3][0], s[0][1], s[1][2], s[2][3]],
    ];
}

fn mix_columns(state: &mut [[u8; 4]; 4]) {
    for word in state {
        let a = *word;
        word[0] = gf_mul(2, a[0]) ^ gf_mul(3, a[1]) ^ a[2] ^ a[3];
        word[1] = a[0] ^ gf_mul(2, a[1]) ^ gf_mul(3, a[2]) ^ a[3];
        word[2] = a[0] ^ a[1] ^ gf_mul(2, a[2]) ^ gf_mul(3, a[3]);
        word[3] = gf_mul(3, a[0]) ^ a[1] ^ a[2] ^ gf_mul(2, a[3]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_key() {
        for _ in 0..100 {
            assert_ne!(random_key(), random_key(), "generated identical keys");
        }
    }

    #[test]
    fn test_mix_columns() {
        // test cases from https://en.wikipedia.org/wiki/Rijndael_MixColumns
        let mut test1: [[u8; 4]; 4] = [
            [0x63, 0x47, 0xa2, 0xf0],
            [0xf2, 0x0a, 0x22, 0x5c],
            [0x01, 0x01, 0x01, 0x01],
            [0xc6, 0xc6, 0xc6, 0xc6],
        ];
        let mut test2: [[u8; 4]; 4] = [
            [0x01, 0x01, 0x01, 0x01],
            [0xc6, 0xc6, 0xc6, 0xc6],
            [0xd4, 0xd4, 0xd4, 0xd5],
            [0x2d, 0x26, 0x31, 0x4c]
        ];

        mix_columns(&mut test1);
        mix_columns(&mut test2);

        assert_eq!(
            test1,
            [[0x5d, 0xe0, 0x70, 0xbb],
            [0x9f, 0xdc, 0x58, 0x9d],
            [0x01, 0x01, 0x01, 0x01],
            [0xc6, 0xc6, 0xc6, 0xc6]],
            "mix columns test case 1 does not match"
        );
        assert_eq!(
            test2,
            [[0x01, 0x01, 0x01, 0x01],
            [0xc6, 0xc6, 0xc6, 0xc6],
            [0xd5, 0xd5, 0xd7, 0xd6],
            [0x4d, 0x7e, 0xbd, 0xf8]],
            "mix columns test case 2 does not match"
        );
    }

    #[test]
    fn test_encrypt_block() {
        // test case from:
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core256.pdf
        let key: [u8; 32] = [
            0x60, 0x3D, 0xEB, 0x10, 0x15, 0xCA, 0x71, 0xBE, 0x2B, 0x73, 0xAE, 0xF0, 0x85, 0x7D,
            0x77, 0x81, 0x1F, 0x35, 0x2C, 0x07, 0x3B, 0x61, 0x08, 0xD7, 0x2D, 0x98, 0x10, 0xA3,
            0x09, 0x14, 0xDF, 0xF4,
        ];
        let plaintext: [[u8; 4]; 4] = [
            [0x6B, 0xC1, 0xBE, 0xE2],
            [0x2E, 0x40, 0x9F, 0x96],
            [0xE9, 0x3D, 0x7E, 0x11],
            [0x73, 0x93, 0x17, 0x2A],
        ];
        let expected: [[u8; 4]; 4] = [
            [0xF3, 0xEE, 0xD1, 0xBD],
            [0xB5, 0xD2, 0xA0, 0x3C],
            [0x06, 0x4B, 0x5A, 0x7E],
            [0x3D, 0xB1, 0x81, 0xF8],
        ];

        let round_keys = expand_key(&key);
        let actual = encrypt_block(&plaintext, &round_keys);

        assert_eq!(actual, expected, "encryption of block failed");
    }
}
