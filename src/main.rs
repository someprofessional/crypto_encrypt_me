use std::io::{stdin, stdout, Write};

use encrypt::file_encrypter;
use decrypt::file_decrypter;

pub mod encrypt;
pub mod decrypt;

fn main() {
    println!("Hello dear user !");
    let mut s = String::new();
    let mut pwd = String::new();
    let _= stdout().flush();
    let mut choice = false;


    while !choice {
        println!("Do you want to 1.encrypt or 2.decrypt your file ? choose a number");
        
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        let trimmed_input = s.trim();

        if trimmed_input == "1" {
            choice = true;
            println!("encryption it is. Choose a password to keep secret :");
            stdin().read_line(&mut pwd).expect("password stupid");
            let trimmed_pwd = pwd.trim();
            file_encrypter(trimmed_pwd);
        } else if trimmed_input == "2" {
            choice = true;
            println!("You want to decrypt your files, alright, let's gooooooooooo");
            file_decrypter();
        } else  {
            println!("something is wrong in your input")
        }
       
    }
}
