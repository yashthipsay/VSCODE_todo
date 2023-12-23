
use std::fs::File;
// Create a new file
pub mod create_new_file{
    use std::{io, fs::File};
    pub fn create() -> Option<File>{
        println!("Create new file: Y for Yes and N for No");
         let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
         if input == "Y" || input == "y"{
            println!("Enter file name");
            let mut file_name = String::new();
            io::stdin().read_line(&mut file_name).unwrap();
            let f = super::File::create(file_name).expect("Unable to create file");
            Some(f)
            
         }else{
            None
         }

    }
}