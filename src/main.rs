use actix_http::{HttpService, KeepAlive};
use actix_server::Server;
use actix_web::dev::PayloadStream;
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::{
    client::Client, error, middleware, web, App, Either, Error, HttpRequest, HttpResponse,
};
use bytes::Bytes;
use futures::sync::mpsc::UnboundedReceiver;
use futures::{
    sink::{Sink, Wait},
    stream,
    sync::mpsc,
    Future, IntoFuture, Stream,
};
use std::io;
use std::thread;
use std::time;

mod controllers;
mod db;
mod handlers;
mod mappers;
mod models;
mod traits;

const SERVER_ADDR: &str = "0.0.0.0:3003";
const APP_NAME: &str = "CMS";

fn main() -> std::io::Result<()> {
    let sys = actix_rt::System::new(APP_NAME);
    Server::build()
        .backlog(1024)
        .bind(APP_NAME, SERVER_ADDR, || {
            HttpService::build().keep_alive(KeepAlive::Os).h1(App::new()
                .data(Client::new())
                .wrap(middleware::Compress::new(ContentEncoding::Gzip))
                .service(web::scope("/").configure(controllers::routes)))
        })
        .unwrap()
        .start();

    println!("Started http server: {}", SERVER_ADDR);
    sys.run()
}
