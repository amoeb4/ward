extern crate aes;
extern crate block_modes;
extern crate block_padding;
extern crate rand;
extern crate hex;

use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::str;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let key: [u8; 16] = rand::thread_rng().gen();
    println!("Generated key: {}", hex::encode(key));
    let iv: [u8; 16] = rand::thread_rng().gen();
    println!("Generated IV: {}", hex::encode(iv));
    let data = b"Secret message";
    println!("Original data: {:?}", str::from_utf8(data).unwrap());
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);
    println!("Encrypted data: {}", hex::encode(&ciphertext));
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(&ciphertext).unwrap();
    println!("Decrypted data: {:?}", str::from_utf8(&decrypted_data).unwrap());
}
