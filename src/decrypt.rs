

pub fn file_decrypter(){
    let path = "./public/";
    let entries = std::fs::read_dir(path).unwrap();
    for entry in entries {
        match entry {
            Ok(entry) => {
                println!("processing entry : {:?}", entry);
                //decrypt here
                println!("Please tell me your password :");

                if true {

                    println!("Thank you\n now please wait until the end of the decryption ...");
                    
                    println!("Decrypted !");
                } else {
                    println!("Task failed successfully !");
                    println!("Well, are you really the one who encrypted it ?");
                }
            }
            Err(e) => {
                println!("  /decrypt/ entry error : {:?}", e);
            }
        }
    }
}