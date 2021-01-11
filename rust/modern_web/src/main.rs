use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::Request;
use async_graphql_actix_web::Response;

mod graphql_schema;
use crate::graphql_schema::{create_schema, SchemaGraphQL};

// #[actix_web::main]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(index_playground)
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/")]
async fn index(schema: web::Data<SchemaGraphQL>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[get("/")]
async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}
