use std::vec;

use super::error::Result;
use super::constants::SBOX;
use super::key::{add_round_key, expand_key};
use super::util::{blockify, pad, gf_mul};

// start with ECB chaining
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let round_keys = expand_key(&key)?;
    let plaintext = blockify(pad(plaintext))?;

    let mut ciphertext: Vec<u8> = vec![];
    for block in plaintext {
        let mut enc_block = encrypt_block(&block, &round_keys)
            .into_iter()
            .flatten()
            .collect();
        ciphertext.append(&mut enc_block);
    }

    Ok(ciphertext)
}

pub(crate) fn encrypt_block(plaintext: &[[u8; 4]; 4], round_keys: &[[[u8; 4]; 4]]) -> [[u8; 4]; 4] {
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

pub(crate) fn sub_bytes(state: &mut [[u8; 4]; 4]) {
    for word in state {
        for byte in word {
            *byte = SBOX[*byte as usize];
        }
    }
}

pub(crate) fn shift_rows(state: &mut [[u8; 4]; 4]) {
    let s = *state;
    *state = [
        [s[0][0], s[1][1], s[2][2], s[3][3]],
        [s[1][0], s[2][1], s[3][2], s[0][3]],
        [s[2][0], s[3][1], s[0][2], s[1][3]],
        [s[3][0], s[0][1], s[1][2], s[2][3]],
    ];
}

pub(crate) fn mix_columns(state: &mut [[u8; 4]; 4]) {
    for word in state {
        let a = *word; // make temp copy of word
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
            [0x2d, 0x26, 0x31, 0x4c],
        ];

        mix_columns(&mut test1);
        mix_columns(&mut test2);

        assert_eq!(
            test1,
            [
                [0x5d, 0xe0, 0x70, 0xbb],
                [0x9f, 0xdc, 0x58, 0x9d],
                [0x01, 0x01, 0x01, 0x01],
                [0xc6, 0xc6, 0xc6, 0xc6]
            ],
            "mix columns test case 1 does not match"
        );
        assert_eq!(
            test2,
            [
                [0x01, 0x01, 0x01, 0x01],
                [0xc6, 0xc6, 0xc6, 0xc6],
                [0xd5, 0xd5, 0xd7, 0xd6],
                [0x4d, 0x7e, 0xbd, 0xf8]
            ],
            "mix columns test case 2 does not match"
        );
    }

    #[test]
    fn test_encrypt_block_256() -> Result<()> {
        // test case from:
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core256.pdf
        let key: [u8; 32] = [
            0x60, 0x3D, 0xEB, 0x10, 0x15, 0xCA, 0x71, 0xBE, //
            0x2B, 0x73, 0xAE, 0xF0, 0x85, 0x7D, 0x77, 0x81, //
            0x1F, 0x35, 0x2C, 0x07, 0x3B, 0x61, 0x08, 0xD7, //
            0x2D, 0x98, 0x10, 0xA3, 0x09, 0x14, 0xDF, 0xF4, //
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

        let round_keys = expand_key(&key)?;
        let actual = encrypt_block(&plaintext, &round_keys);

        assert_eq!(actual, expected, "incorrect AES-256 encryption of block");

        Ok(())
    }

    #[test]
    fn test_encrypt_block_192() -> Result<()> {
        // test case from:
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core192.pdf
        let key: [u8; 24] = [
            0x8E, 0x73, 0xB0, 0xF7, 0xDA, 0x0E, 0x64, 0x52, //
            0xC8, 0x10, 0xF3, 0x2B, 0x80, 0x90, 0x79, 0xE5, //
            0x62, 0xF8, 0xEA, 0xD2, 0x52, 0x2C, 0x6B, 0x7B, //
        ];

        let plaintext: [[u8; 4]; 4] = [
            [0x6B, 0xC1, 0xBE, 0xE2],
            [0x2E, 0x40, 0x9F, 0x96],
            [0xE9, 0x3D, 0x7E, 0x11],
            [0x73, 0x93, 0x17, 0x2A],
        ];

        let expected: [[u8; 4]; 4] = [
            [0xBD, 0x33, 0x4F, 0x1D],
            [0x6E, 0x45, 0xF2, 0x5F],
            [0xF7, 0x12, 0xA2, 0x14],
            [0x57, 0x1F, 0xA5, 0xCC],
        ];

        let round_keys = expand_key(&key)?;
        let actual = encrypt_block(&plaintext, &round_keys);

        assert_eq!(actual, expected, "incorrect AES-192 encryption of block");

        Ok(())
    }

    #[test]
    fn test_encrypt_block_128() -> Result<()> {
        // test case from:
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core128.pdf
        let key: [u8; 16] = [
            0x2B, 0x7E, 0x15, 0x16, 0x28, 0xAE, 0xD2, 0xA6, //
            0xAB, 0xF7, 0x15, 0x88, 0x09, 0xCF, 0x4F, 0x3C, //
        ];

        let plaintext: [[u8; 4]; 4] = [
            [0x6B, 0xC1, 0xBE, 0xE2],
            [0x2E, 0x40, 0x9F, 0x96],
            [0xE9, 0x3D, 0x7E, 0x11],
            [0x73, 0x93, 0x17, 0x2A],
        ];

        let expected: [[u8; 4]; 4] = [
            [0x3A, 0xD7, 0x7B, 0xB4],
            [0x0D, 0x7A, 0x36, 0x60],
            [0xA8, 0x9E, 0xCA, 0xF3],
            [0x24, 0x66, 0xEF, 0x97],
        ];

        let round_keys = expand_key(&key)?;
        let actual = encrypt_block(&plaintext, &round_keys);

        assert_eq!(actual, expected, "incorrect AES-128 encryption of block");

        Ok(())
    }

    #[test]
    fn test_encrypt_256() -> Result<()> {
        // test case from:
        // // test case from:
        // https://csrc.nist.gov/CSRC/media/Projects/Cryptographic-Standards-and-Guidelines/documents/examples/AES_Core256.pdf
        let key: [u8; 32] = [
            0x60, 0x3D, 0xEB, 0x10, 0x15, 0xCA, 0x71, 0xBE, //
            0x2B, 0x73, 0xAE, 0xF0, 0x85, 0x7D, 0x77, 0x81, //
            0x1F, 0x35, 0x2C, 0x07, 0x3B, 0x61, 0x08, 0xD7, //
            0x2D, 0x98, 0x10, 0xA3, 0x09, 0x14, 0xDF, 0xF4, //
        ];

        let plaintext: [u8; 58] = [
            0x6B, 0xC1, 0xBE, 0xE2, 0x2E, 0x40, 0x9F, 0x96, //
            0xE9, 0x3D, 0x7E, 0x11, 0x73, 0x93, 0x17, 0x2A, //
            0xAE, 0x2D, 0x8A, 0x57, 0x1E, 0x03, 0xAC, 0x9C, //
            0x9E, 0xB7, 0x6F, 0xAC, 0x45, 0xAF, 0x8E, 0x51, //
            0x30, 0xC8, 0x1C, 0x46, 0xA3, 0x5C, 0xE4, 0x11, //
            0xE5, 0xFB, 0xC1, 0x19, 0x1A, 0x0A, 0x52, 0xEF, //
            0xF6, 0x9F, 0x24, 0x45, 0xDF, 0x4F, 0x9B, 0x17, //
            0xAD, 0x2B,
        ];

        let expected: [u8; 64] = [
            0xF3, 0xEE, 0xD1, 0xBD, 0xB5, 0xD2, 0xA0, 0x3C, //
            0x06, 0x4B, 0x5A, 0x7E, 0x3D, 0xB1, 0x81, 0xF8, //
            0x59, 0x1C, 0xCB, 0x10, 0xD4, 0x10, 0xED, 0x26, //
            0xDC, 0x5B, 0xA7, 0x4A, 0x31, 0x36, 0x28, 0x70, //
            0xB6, 0xED, 0x21, 0xB9, 0x9C, 0xA6, 0xF4, 0xF9, //
            0xF1, 0x53, 0xE7, 0xB1, 0xBE, 0xAF, 0xED, 0x1D, //
            0xEB, 0xCB, 0xB2, 0x68, 0x53, 0x40, 0x56, 0x49, //
            0x50, 0x88, 0xBC, 0xCC, 0x9D, 0xEF, 0x97, 0xB1, //
        ];

        let actual = encrypt(&plaintext, &key)?;

        assert_eq!(
            actual, expected,
            "incorrect AES-256 encryption of 58 bytes of plaintext"
        );

        Ok(())
    }
}
