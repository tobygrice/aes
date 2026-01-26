use std::vec;

use rand::rngs::OsRng;
use rand::TryRngCore;

use super::shared::expand_key;

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
    // round 1
    // for key in roundkeys
    //      do encryption
    // last round
    
    
    let mut ciphertext = vec![];
    ciphertext
}

fn random_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    OsRng.try_fill_bytes(&mut key).expect("key generation failed");
    key
}

// fn add_round_key(state: &mut [[u8; 4]; 4], round_key: &[[u8; 4]; 4]) {}

// fn sub_bytes(state: &mut [[u8; 4]; 4]) {}

// fn shift_rows(state: &mut [[u8; 4]; 4]) {}

// fn mix_columns(state: &mut [[u8; 4]; 4]) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_ne!(random_key(), random_key(), "generated identical keys");
        println!("{:?}", random_key());
        println!("{:?}", random_key());
    }
}
