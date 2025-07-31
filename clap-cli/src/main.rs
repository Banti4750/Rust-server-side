use clap::{Arg, Command};
#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn say_hello() -> &'static str {
    "Hello dear world!"
}
#[tokio::main]
async fn main() {
    let matches = Command::new("clap-cli")
        .version("0.1.0")
        .about("This is a test CLI")
        .arg(Arg::new("name").help("Your name").required(false).index(1))
        .arg(Arg::new("greet").short('g').long("greet").help("Greet the user"))
        .get_matches();

    // let nn = matches.get_one::<String>("name");
    // let gerret = matches.get_flag("greet");


    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            println!("Received a request");
        }
    });

     server.utilize(router! {
        get "/get" => |_req, _res| {
            say_hello()
        }
    });

    println!("Starting server on http://127.0.0.1:6767");
    server.listen("127.0.0.1:6767").await.unwrap();
}
