use std::io;
use obfuscate::{create_export_file, check_commands, write_to_export_file, Commands};

fn main() {

    
    println!(
        "\
.___________________________________________________________________.\n\
|                                                                   |\n\
|                                                                   |\n\
|                             Obfuscate                             |\n\
|                                                                   |\n\
|                                                                   |\n\
|           'quit'            'create'            'help'            |\n\
|         the program     a new empty file    display commands      |\n\
|___________________________________________________________________|\n\
|                                                                   |\n\
|             Type below to append the text to the file             |\n\
|___________________________________________________________________|\n\
"
    );

    loop {

        let path: &str = "export.txt";

        let mut content: String = String::new();

        io::stdin()
            .read_line(&mut content)
            .expect("Failed to read string");

        match check_commands(&content){
            Commands::Quit => break,
            Commands::Create => {
                create_export_file(path);
            },
            Commands::Write => {
                write_to_export_file(path, &content);
            },
            Commands::Help => {
                println!(
                    "\
.___________________________________________________________________.\n\
|                                                                   |\n\
|                                                                   |\n\
|                             Obfuscate                             |\n\
|                                                                   |\n\
|                                                                   |\n\
|           'quit'            'create'            'help'            |\n\
|         the program     a new empty file    display commands      |\n\
|___________________________________________________________________|\n\
|                                                                   |\n\
|             Type below to append the text to the file             |\n\
|___________________________________________________________________|\n\
                    "
                );
            },
        };
    }
}
