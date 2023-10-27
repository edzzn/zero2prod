use crate::configuration::get_configuration;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let _guard = sentry::init((configuration.sentry_url, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
      }));

    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(sentry_actix::Sentry::new())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
