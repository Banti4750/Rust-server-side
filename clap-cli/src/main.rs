use clap::{Arg, Command};

fn main() {
    let matches = Command::new("clap-cli")
        .version("0.1.0")
        .about("This is a test CLI")
        .arg(Arg::new("name").help("Your name").required(false).index(1))
        .get_matches();

    if let Some(name) = matches.get_one::<String>("name") {
        println!("Hello, {}!", name);
    } else {
        println!("Hello, world!");
    }

    println!("CLI parsed successfully!");
}
