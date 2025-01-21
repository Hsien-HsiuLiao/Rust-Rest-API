#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer}; //https://docs.rs/actix-web/latest/actix_web/struct.HttpServer.html
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod employees;
mod error_handler;
mod schema;

//https://docs.rs/actix-rt/2.10.0/actix_rt/attr.main.html
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //https://docs.rs/dotenv/0.15.0/dotenv/index.html#types
    dotenv().ok();  //https://doc.rust-lang.org/std/result/index.html#transforming-contained-values
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
