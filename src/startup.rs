use crate::routes::{health_check, subscribe};
use actix_web::{App, HttpServer, dev};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<dev::Server> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}
