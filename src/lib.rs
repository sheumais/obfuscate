use std::io::Write;
use std::fs::{File, OpenOptions};

pub fn create_export_file(path: &str) -> File {
    let mut file = File::create(path).ok().unwrap();
    file.write_all(&[0xFE, 0xFF]).ok();
    file
}

pub fn write_to_export_file(path: &str, data: &str) {
    let mut file = match OpenOptions::new()
        .append(true)
        .open(path){
            Ok(f) => f,
            Err(_) => create_export_file(path),
        };
    file.write_all(data.as_bytes()).expect("Unable to write data");
}

pub fn check_commands(s: &str) -> Commands {
    match s.trim() {
        "quit" => Commands::Quit,
        "create" => Commands::Create,
        "help" => Commands::Help,
        _ => Commands::Write,
    }
}

pub enum Commands{
    Quit,
    Write,
    Create,
    Help
}
