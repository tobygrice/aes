// Multiplication in the Galois finite field GF(2^8)
// - adapted from https://crypto.stackexchange.com/a/71206
// - unfortunately, Rust doesn't allow such pretty bit manipulation on u8s,
//   so the translation is not as clean
pub fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    while b > 0 {
        if (b & 1) != 0 {
            p ^= a; // add a to p if the lowest bit of b is set
        }

        // multiply a by 2 in the Galois finite field
        // overflow -> reduce modulo GF(2^8) polynomial x^8 + x^4 + x^3 + x + 1
        // ^= 0x1B after shifting
        let hi = a & 0x80;
        a <<= 1; // multiply by 2
        if hi != 0 {
            a ^= 0x1B; // reduce if overflow
        }

        b >>= 1;
    }
    p
}

// this function was written with assistance of an LLM
pub fn blockify(plaintext: &[u8]) -> Vec<[[u8; 4]; 4]> {
    let mut buf = plaintext.to_vec();

    // PKCS#7: pad with number of elems to pad
    let pad = 16 - (buf.len() % 16);
    buf.extend(std::iter::repeat(pad as u8).take(pad));

    // better not to use unwrap() ?
    buf.chunks_exact(16)
        .map(|c| {
            [
                c[0..4].try_into().unwrap(),
                c[4..8].try_into().unwrap(),
                c[8..12].try_into().unwrap(),
                c[12..16].try_into().unwrap(),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockify_formatting() {
        // 20 bytes -> pads to 32 bytes, so 2 blocks/states.
        let plaintext: [u8; 20] = [
            0x6B, 0xC1, 0xBE, 0xE2, //
            0x2E, 0x40, 0x9F, 0x96, //
            0xE9, 0x3D, 0x7E, 0x11, //
            0x73, 0x93, 0x17, 0x2A, //
            0xAE, 0x2D, 0x8A, 0x57, //
        ];

        let expected1: Vec<[[u8; 4]; 4]> = vec![
            [
                [0x6B, 0xC1, 0xBE, 0xE2],
                [0x2E, 0x40, 0x9F, 0x96],
                [0xE9, 0x3D, 0x7E, 0x11],
                [0x73, 0x93, 0x17, 0x2A],
            ],
            [
                [0xAE, 0x2D, 0x8A, 0x57],
                [0x0C, 0x0C, 0x0C, 0x0C],
                [0x0C, 0x0C, 0x0C, 0x0C],
                [0x0C, 0x0C, 0x0C, 0x0C],
            ],
        ];

        let expected2: Vec<[[u8; 4]; 4]> = vec![[
            [0x6B, 0xC1, 0xBE, 0xE2],
            [0x2E, 0x40, 0x9F, 0x96],
            [0xE9, 0x3D, 0x06, 0x06],
            [0x06, 0x06, 0x06, 0x06],
        ]];

        let actual1 = blockify(&plaintext);
        let actual2 = blockify(&plaintext[..10]);

        assert_eq!(actual1, expected1);
        assert_eq!(actual2, expected2);
    }
}
