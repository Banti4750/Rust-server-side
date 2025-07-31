use std::fs::File;
use std::io::{Write, Result};

fn log_something(filename: String, contents: String) -> Result<()> {
    let mut f = File::create(filename)?;                  
    f.write_all(contents.as_bytes())?;                    
    Ok(())
}

fn main() {
    match log_something("log.txt".to_string(), "It's me what is your name ".to_string()) {
        Ok(()) => println!("File created"),
        Err(_) => println!("Error: could not create file."),
    }
}
