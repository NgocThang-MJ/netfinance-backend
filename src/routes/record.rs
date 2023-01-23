use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::models::record::Record;

#[get("/records")]
pub async fn get_records(db: Data<Database>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let cursor = records_coll.find(None, None).await;

    match cursor {
        Ok(cursor) => {
            let records: Vec<Record> = cursor.try_collect().await.unwrap();
            HttpResponse::Ok().json(records)
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/records/{id}")]
pub async fn get_record(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let obj_id = ObjectId::parse_str(id.into_inner()).unwrap();
    let filter = doc! {"_id": obj_id};

    let result = records_coll.find_one(filter, None).await;

    match result {
        Ok(Some(record)) => HttpResponse::Ok().json(record),
        Ok(None) => HttpResponse::NotFound().body("Record not found!"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/records/create")]
pub async fn create_record(db: Data<Database>, new_record: Json<Record>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let result = records_coll.insert_one(new_record.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("record created"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/records/update/{id}")]
pub async fn update_record(
    db: Data<Database>,
    id: Path<String>,
    new_record: Json<Record>,
) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let obj_id = ObjectId::parse_str(id.into_inner()).unwrap();
    let filter = doc! {"_id": obj_id};

    let a = doc! {
        "$set": {
            "name": &new_record.name,
            "price": &new_record.price,
            "date": &new_record.date,
            "attribute": new_record.attribute.to_string(),
        }
    };

    let result = records_coll.update_one(filter, a, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Record updated"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/records/delete/{id}")]
pub async fn delete_record(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let obj_id = ObjectId::parse_str(id.into_inner()).unwrap();
    let filter = doc! {"_id": obj_id};

    let result = records_coll.delete_one(filter, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Deleted record"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
