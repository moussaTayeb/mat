use std::env;
use std::process;

use mat::Command;

fn main() {    
    let content: Command = Command::new(env::args()).unwrap_or_else(|err| {
        eprintln!("error {}", err);
        process::exit(1);
    });
    let n: String = mat::read_file_content(content.file.clone()).unwrap_or_else(|err| {
        eprintln!("error {}", err);
        process::exit(1);
    });
    println!("{}", n);

}