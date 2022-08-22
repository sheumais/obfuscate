use std::{fs, io};
use std::io::Write;
use std::fs::File;

fn create_export_file() -> io::Result<File> {
    let mut file = File::create("export.txt")?;
    file.write_all(&[0xFE, 0xFF])?;
    Ok(file)
}

fn write_obfuscation(mut file: fs::File, contents: String) {
    file.write_all(&contents.as_bytes()).ok();}

pub fn get_string() {
       loop {

        println!("Please enter your string");

        let mut stringo = String::new();

        io::stdin()
            .read_line(&mut stringo)
            .expect("Failed to read string");

        let stringo: String = match stringo.trim().parse::<String>() {
            Ok(str) => str,
            Err(_) => continue,
        };
        let file = create_export_file().unwrap();
        write_obfuscation(file, stringo);
        println!("Successfully exported obfuscated string to export.txt");
        break;
    }
}

pub fn get_file() {
    loop {
        println!("Please enter the file name");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read file name");
        let file_name: String = match file_name.trim().parse::<String>() {
            Ok(str) => str,
            Err(_) => continue,
        };
        let contents = fs::read_to_string(&file_name);
        let _contents = match contents {
            Ok(contents) => {
                let file = create_export_file().unwrap();
                write_obfuscation(file, contents);
                println!("Successfully exported obfuscated file to export.txt");
                break;
            }
            Err(error) => println!("Problem reading the file: {:?}", error),
        };
    }
}