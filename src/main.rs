use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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
    let db = connect_to_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(hello)
            .service(create_record)
            .service(get_records)
            .service(get_record)
            .service(delete_record)
            .service(update_record)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
