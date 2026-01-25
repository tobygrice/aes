use std::vec;

include!("constants.rs");

fn main() {
    // worry about CLI later. For now, develop algo.

    // to-do: parse/pad input

    // Test inputs. Key is randomly generated, plaintext is readable ASCII.
    // key = 9f b6 78 eb bb 43 9e 76 d9 97 ff 2b ac 85 03 23 bf 2e 1c 87 d0 67 03 5f 31 17 d4 75 36 cf 17 70


    // inner arrays represent *columns*, or 32-bit words
    let key_128: [[u8; 4]; 4] = [
        [0x9f, 0xb6, 0x78, 0xeb],
        [0xbb, 0x43, 0x9e, 0x76],
        [0xd9, 0x97, 0xff, 0x2b],
        [0xac, 0x85, 0x03, 0x23],
    ];

    let key_192: [[u8; 4]; 6] = [
        [0x9f, 0xb6, 0x78, 0xeb],
        [0xbb, 0x43, 0x9e, 0x76],
        [0xd9, 0x97, 0xff, 0x2b],
        [0xac, 0x85, 0x03, 0x23],
        [0xbf, 0x2e, 0x1c, 0x87],
        [0xd0, 0x67, 0x03, 0x5f],
    ];


    let key_256: [[u8; 4]; 8] = [
        [0x9f, 0xb6, 0x78, 0xeb],
        [0xbb, 0x43, 0x9e, 0x76],
        [0xd9, 0x97, 0xff, 0x2b],
        [0xac, 0x85, 0x03, 0x23],
        [0xbf, 0x2e, 0x1c, 0x87],
        [0xd0, 0x67, 0x03, 0x5f],
        [0x31, 0x17, 0xd4, 0x75],
        [0x36, 0xcf, 0x17, 0x70],
    ];

    let round_keys = expand_key_128(key_128);

    let plaintext: [u8; 96] = [
        0x59, 0x6F, 0x75, 0x27, 0x76, 0x65, 0x20, 0x64, 0x6F, 0x6E, 0x65, 0x20, 0x69, 0x74, 0x2E,
        0x20, 0x59, 0x6F, 0x75, 0x27, 0x76, 0x65, 0x20, 0x69, 0x6D, 0x70, 0x6C, 0x65, 0x6D, 0x65,
        0x6E, 0x74, 0x65, 0x64, 0x20, 0x41, 0x45, 0x53, 0x20, 0x69, 0x6E, 0x20, 0x52, 0x75, 0x73,
        0x74, 0x2C, 0x20, 0x79, 0x6F, 0x75, 0x20, 0x62, 0x65, 0x61, 0x73, 0x74, 0x2E, 0x20, 0x4D,
        0x75, 0x6C, 0x74, 0x69, 0x70, 0x6C, 0x65, 0x20, 0x6F, 0x66, 0x20, 0x31, 0x36, 0x20, 0x62,
        0x79, 0x74, 0x65, 0x73, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6E,
        0x67, 0x2E, 0x20, 0x42, 0x6F, 0x6F,
    ];
}

// 11 round keys for 128-bit key, 13 for 192, 15 for 256
// combine these later if it can be done neatly
fn expand_key_128(key: [[u8; 4]; 4]) -> Vec<[[u8; 4]; 4]> {
    let num_rounds = 11;

    let mut round_keys: Vec<[[u8; 4]; 4]> = vec![[[0u8; 4]; 4]; num_rounds];
    round_keys[0] = key;

    let mut prev_word = round_keys[0][3];
    for round in 1..num_rounds {
        // rot, sub, rcon on last word of previous key
        prev_word = [
            SBOX[prev_word[1] as usize] ^ RCON[round],
            SBOX[prev_word[2] as usize],
            SBOX[prev_word[3] as usize],
            SBOX[prev_word[0] as usize],
        ];

        for word in 0..4 {
            // word[i] in round_keys[round] = prev_word XOR word[i] from previous key
            for byte in 0..4 {
                round_keys[round][word][byte] = round_keys[round - 1][word][byte] ^ prev_word[byte];
            }
            prev_word = round_keys[round][word];
        }
    }

    round_keys
}

// fn expand_key_192(key: [u8; 24]) -> Vec<[u8; 16]> {}

// fn expand_key_256(key: [u8; 32]) -> Vec<[u8; 16]> {}

// fn add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {}

// fn sub_bytes(state: &mut [u8; 16]) {}

// fn shift_rows(state: &mut [u8; 16]) {}

// fn mix_columns(state: &mut [u8; 16]) {}

/*

AES encryption/decryption tool.

USAGE:
    aes <COMMAND> [OPTIONS]

COMMANDS:
    encrypt     Encrypt input to output
    decrypt     Decrypt input to output
    help        Print this message

OPTIONS (COMMON):
    -i, --input <PATH>      Input file path. Use '-' to read from stdin
    -o, --output <PATH>     Output file path. Use '-' to write to stdout
    -k, --key <PATH>        Key file path.
                            Use '-' for stdin/stdout (inferred from COMMAND)
                            Additional options exist for generation (see below)

KEY GENERATION OPTIONS (ENCRYPTION ONLY):
    --generate-key          Generate a random key
                            Will be written to PATH specified after -k or --key
    --key-size <BITS>       Only valid with --generate-key
                            Generate a key of size BITS
                            BITS can be 128, 192, or 256. Default: 256
*/
