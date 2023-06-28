use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    /*
    HttpServer::new does not take App as argument - it wants a closure that returns an App struct.
    This is to support actix-web’s runtime model: actix-web will spin up a worker
    process for each available core on your machine.
    Each worker runs its own copy of the application built by HttpServer calling
    the very same "closure" that HttpServer::new takes as argument.

    That is why connection has to be cloneable
    PgConnection does not implement Clone because it sits on top of a non-cloneable system resource,
    a TCP connection with Postgres.
    */
    let db_pool = web::Data::new(db_pool);
    /*
    web::Data wraps our connection in an Atomic Reference Counted pointer,
    an Arc: each instance of the application, instead of getting a raw copy of a PgConnection,
    will get a pointer to one.
    */

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
