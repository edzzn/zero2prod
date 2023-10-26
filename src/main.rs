use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configurations;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configurations().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    

    let listener =
        TcpListener::bind(address).expect("Error binding port to listener");

    run(listener)?.await
}
