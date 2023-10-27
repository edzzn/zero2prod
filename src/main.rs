use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configurations;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configurations().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let connection_pool =  PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    print!("address: {}", &address);
    let listener = TcpListener::bind(address).expect("Error binding port to listener");

    run(listener, connection_pool)?.await
}
