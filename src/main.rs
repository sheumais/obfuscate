use std::env;
use std::fs;
use std::io::Write;

fn write_obfuscation(mut file: fs::File, contents: &String) {
    file.write_all(&[0xFE, 0xFF]).ok();
    file.write_all(&contents.as_bytes()).ok();}

fn main() {
    let args: Vec<String> = env::args().collect();    
    let length = args.len();
    if length == 2 {
        let file = fs::File::create("export.txt").expect("Unable to create file");
        let contents = &args[1];
        write_obfuscation(file, &contents);}
    if length == 3 {
        if &args[1] == "file" {
            let file = fs::File::create("export.txt").expect("Unable to create file");
            let contents = &args[2];
            write_obfuscation(file, &contents);}
        else { println!("Please supply a file"); }}
    if length == 4 {
        if &args[1] == "file" {
            let contents = fs::read_to_string(&args[2]).expect("Something went wrong reading the file");
            let file = fs::File::create(&args[3]).expect("Unable to create file");
            write_obfuscation(file, &contents);}
        else { println!("Please supply a file"); }}
    else {println!("Please supply appropriate arguments");}}
