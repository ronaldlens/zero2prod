use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .expect("Could not bind to 127.0.0.1:3000");

    run(listener)?.await

}