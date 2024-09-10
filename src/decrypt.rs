use aes_gcm::aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm::Aes256Gcm;
use aes_gcm_siv::Nonce;
use std::fs::File;
use std::io::{Read, Write};
use std::io::{self, BufRead};

const NONCE_SIZE: usize = 12;

pub fn file_decrypter() {
    let path = "./public/";
    let entries = std::fs::read_dir(path).unwrap();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("encrypted") {
                    println!("processing entry : {:?}", entry);

                    let mut file = File::open(&path).expect("failed to open encrypted file");
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).expect("Failed to read encypted file");

                    //decrypt here
                    println!("Please tell me your password :");
                    let password = read_password();

                    let key = derive_key_from_password(&password);
                    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

                    let decrypted_content = decrypt_content(&cipher, &buffer);


                    match decrypted_content {
                        Ok(content) => {
                            // Write the decrypted content back to a new file
                            let mut decrypted_file = File::create(path.with_extension("decrypted")).expect("Failed to create decrypted file");
                            decrypted_file.write_all(&content).expect("Failed to write decrypted content");

                            println!("File decrypted and saved: {:?}", path.with_extension("decrypted"));
                        }
                        Err(e) => {
                            println!("Decryption failed successfuly : {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  /decrypt/ entry error : {:?}", e);
            }
        }
    }
}

fn read_password() -> String {
    let stdin = io::stdin();
    let mut password = String::new();
    stdin.lock().read_line(&mut password).expect("Failed to read password");
    password.trim().to_string()
}

fn derive_key_from_password(password: &str) -> [u8; 32] {
    use sha2::{Digest, Sha256}; 
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}

fn decrypt_content(cipher: &Aes256Gcm, encrypted_content: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let (nonce_bytes, ciphertext) = encrypted_content.split_at(NONCE_SIZE);

    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext)
}