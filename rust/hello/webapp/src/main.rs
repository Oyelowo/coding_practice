#[macro_use]
extern crate juniper;

use std::{io, sync::Arc};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::future::Future;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod graphql_schema;
use crate::graphql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/grapihql").route(web::get().to(graphiql)))
    })
    .bind("localhost:8080")?
    .run()
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
