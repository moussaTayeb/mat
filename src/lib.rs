use std::env;
use std::error::Error;
use std::fs;

pub struct Command{
    pub query: String,
    pub option: String,
    pub file: String
}

pub fn read_file_content(file: String) -> Result<String, Box<dyn Error>>{
    let foo = fs::read_to_string(file)?;
    Ok(foo)
}


impl Command{
    pub fn new(mut args: env::Args) -> Result<Command, &'static str>{
        args.next();

        let query = match args.next() {
            Some(arg) => if arg == "mat"{
                arg
            }
            else{
                return Err("invalid query")
            },
            None => return Err("Didn't get a query")
        };

        let option = match args.next() {
            Some(arg) => if arg == "-n"{
                arg
            }
            else{
                return Err("invalid option")
            }
            None => return Err("Didn't get an option")
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file")
        };
       
        Ok(Command{query, option, file})
        
    }
}
