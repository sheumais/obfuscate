use std::io;
use obfuscate::{get_string, get_file};

fn main() {

    println!("'file' or 'string'");

    loop {

        let mut file_or_string = String::new();

        io::stdin()
            .read_line(&mut file_or_string)
            .expect("Failed to read line");

        let file_or_string: String = match file_or_string.trim().parse(){
            Ok(str) => str,
            Err(_) => continue,
        };

        match file_or_string.as_str() {
            "file" => {get_file(); break;}
            "string" => {get_string(); break;}
            _ => println!("try again"),
        }
    }
}