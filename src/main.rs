use std::vec;

include!("constants.rs");

fn main() {
    // worry about CLI later. For now, develop algo.

    // to-do: parse/pad input

    // Test inputs. Key is randomly generated, plaintext is readable ASCII.
    // key = 9f b6 78 eb bb 43 9e 76 d9 97 ff 2b ac 85 03 23 bf 2e 1c 87 d0 67 03 5f 31 17 d4 75 36 cf 17 70


    // inner arrays represent *columns*, or 32-bit words
    let key: Vec<[u8; 4]> = vec![
        [0x9f, 0xb6, 0x78, 0xeb],
        [0xbb, 0x43, 0x9e, 0x76],
        [0xd9, 0x97, 0xff, 0x2b],
        [0xac, 0x85, 0x03, 0x23],
        [0xbf, 0x2e, 0x1c, 0x87],
        [0xd0, 0x67, 0x03, 0x5f],
        [0x31, 0x17, 0xd4, 0x75],
        [0x36, 0xcf, 0x17, 0x70],
    ];

    let round_keys = expand_key(&key);

    let _plaintext: [u8; 96] = [
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

fn expand_key(key: &[[u8; 4]]) -> Vec<[[u8; 4]; 4]> {
    let key_size = key.len(); // key size (in 4-byte words), Nk in docs
    let num_keys = key_size + 6 + 1; // num_keys = num_rounds + 1 
    let num_words = num_keys * 4;

    assert!(matches!(key_size, 4 | 6 | 8), "Key is not 128, 192, or 256 bits.");

    // initialise vector to hold expanded keys
    let mut round_words: Vec<[u8; 4]> = vec![[0u8; 4]; num_words];
    
    // fill start of round_words vector with initial key
    round_words[..key_size].copy_from_slice(&key[..key_size]);

    // initialise prev_word variable with last word of initial key
    let mut prev_word = round_words[key_size - 1];
    let mut rcon_iter = 0;
    for word in key_size..num_words {
        if word % key_size == 0 {
            // calculate rot_word, sub_word, and rcon on previous word 
            // do not apply to round_words, only store in prev_word variable
            prev_word = [
                SBOX[prev_word[1] as usize] ^ RCON[rcon_iter],
                SBOX[prev_word[2] as usize],
                SBOX[prev_word[3] as usize],
                SBOX[prev_word[0] as usize],
            ];
            rcon_iter += 1;
        } else if key_size == 8 && word % key_size == 4 { 
            // additional substitution for 256-bit keys only
            prev_word = [
                SBOX[prev_word[0] as usize],
                SBOX[prev_word[1] as usize],
                SBOX[prev_word[2] as usize],
                SBOX[prev_word[3] as usize],
            ];
        }

        // word[i] in round_keys[round] = prev_word XOR word[i] from previous key
        for byte in 0..4 {
            round_words[word][byte] = round_words[word - key_size][byte] ^ prev_word[byte];
        }
        prev_word = round_words[word];
    }


    let mut round_keys: Vec<[[u8; 4]; 4]> = vec![[[0u8; 4]; 4]; num_keys];
    for i in 0..num_words {
        round_keys[i / 4][i % 4] = round_words[i];
    }
    round_keys
}


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
