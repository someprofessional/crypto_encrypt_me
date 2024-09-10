use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm::Aes256Gcm;
use aes_gcm_siv::Nonce;
use rand::RngCore;
use std::fs::File;
use std::io::{Read, Write};

const NONCE_SIZE: usize = 12;

pub fn file_encrypter(password: &str) {
    println!("Your newly set password is: {}", password);

    let key = derive_key_from_password(password);
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    let path = "./public/";
    let entries = std::fs::read_dir(path).unwrap();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    println!("Processing entry: {:?}", path);

                    let mut file = File::open(&path).expect("Failed to open file");
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).expect("Failed to read file");

                    let encrypted_content = encrypt_content(&cipher, &buffer);

                    let mut encrypted_file = File::create(path.with_extension("encrypted")).expect("Failed to create encrypted file");
                    encrypted_file.write_all(&encrypted_content).expect("Failed to write encrypted content");

                    println!("File encrypted and saved: {:?}", path.with_extension("encrypted"));
                }
            }
            Err(e) => {
                println!("/encrypt/ entry error: {:?}", e);
            }
        }
    }
            println!("\n\n      Finished !\n\n");
}

fn derive_key_from_password(password: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256};    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}

fn encrypt_content(cipher: &Aes256Gcm, content: &[u8]) -> Vec<u8> {
    let nonce = generate_nonce();

    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), content)
        .expect("Encryption failed");

    [nonce.to_vec(), ciphertext].concat()
}

fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
}
