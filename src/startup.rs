use crate::routes::{health_check, subscribe};
use actix_web::{App, HttpServer, dev};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection: PgPool) -> std::io::Result<dev::Server> {
    let connection = actix_web::web::Data::new(connection);

    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
