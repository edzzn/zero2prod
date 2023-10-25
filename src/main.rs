use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener =
        TcpListener::bind("http://127.0.0.1:3300").expect("Error binding port to listener");

    run(listener)?.await
}
