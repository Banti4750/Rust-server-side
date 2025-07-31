#[macro_use] extern crate nickel;

use nickel::Nickel;

fn say_hello() -> &'static str {
    "Hello dear world!"
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    println!("Starting server on http://127.0.0.1:6767");
    server.listen("127.0.0.1:6767").await.unwrap();
}