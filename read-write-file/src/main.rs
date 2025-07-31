extern crate chrono;
#[macro_use]
extern crate nickel;

use nickel::Nickel;

use chrono::*;
use std::fs::File;
use std::io::{Result, Write};

fn format_time() -> String {
    let now = chrono::Local::now();
    return now.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
}

fn log_something(filename: String, contents: String) -> Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(contents.as_bytes())?;
    Ok(())
}

fn log_time() -> String {
    let now = chrono::Local::now();
    return now.to_string();
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            let time = format_time();
            log_something("log.txt".to_string(), time.clone()).unwrap();
            log_time()
        }
    });

    server.utilize(router! {
        get "/log" => |_req, _res| {
            let contents = std::fs::read_to_string("log.txt").unwrap();
            contents
        }
    });

    server.utilize(router! {
        get "/time" => |_req, _res| {
            format_time()
        }
    });

    println!("Starting server on http://127.0.0.1:6767");
    server.listen("127.0.0.1:6767").await.unwrap();
}
