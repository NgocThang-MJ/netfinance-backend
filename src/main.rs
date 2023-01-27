extern crate dotenv;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use dotenv::dotenv;
use std::env;

mod models;
mod routes;
mod util;

use routes::hello::hello;
use routes::record::{create_record, delete_record, get_record, get_records, update_record};

use util::db::mongodb::connect_to_db;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = connect_to_db().await;
    let port = env::var("PORT")
        .expect("Can't get port")
        .parse::<u16>()
        .expect("Cannot get port");

    println!("Server running at port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(create_record)
            .service(get_records)
            .service(get_record)
            .service(delete_record)
            .service(update_record)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
