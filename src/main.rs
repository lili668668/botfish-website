extern crate actix_web;

use std::{io};
use actix_web::{middleware, App, HttpServer};
use actix_files as fs;

fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                fs::Files::new("/", "./static").index_file("index.html")
            )
    })
    .bind("0.0.0.0:6002")?
    .run()
}
