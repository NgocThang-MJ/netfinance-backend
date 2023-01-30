use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    // options::FindOptions,
    Database,
};

use crate::models::record::Record;
use crate::types::messages::{DateRangeQuery, ResponseMsg};

#[get("/records")]
pub async fn get_records(db: Data<Database>, query: Query<DateRangeQuery>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");
    // Some default error message
    let error_400 = ResponseMsg {
        msg: String::from("Bad Request"),
    };
    let error_500 = ResponseMsg {
        msg: String::from("Internal Server Error"),
    };

    let mut pipeline = vec![];

    // Check if request have start_date and end_date query
    if query.start_date.is_none() || query.end_date.is_none() {
        return HttpResponse::BadRequest().json(error_400);
    }
    let start_date = query.start_date.clone().unwrap_or_default();
    let end_date = query.end_date.clone().unwrap_or_default();

    pipeline.push(doc! {"$match": {"date": {"$gte": start_date, "$lte": end_date}}});
    pipeline.push(doc! {"$sort": {"date": -1}});

    //let find_options = FindOptions::builder()
    //    .sort(doc! {"date": -1})
    //    .limit(30)
    //    .build();

    // let cursor = records_coll.find(None, find_options).await;
    let cursor = records_coll.aggregate(pipeline, None).await;

    match cursor {
        Ok(cursor) => {
            let records: Vec<Document> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);
            HttpResponse::Ok().json(records)
        }
        Err(_) => HttpResponse::InternalServerError().json(error_500),
    }
}

#[get("/records/{id}")]
pub async fn get_record(db: Data<Database>, id: Path<String>) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let obj_id = match ObjectId::parse_str(id.into_inner()) {
        Ok(objid) => objid,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

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

    let success_msg = ResponseMsg {
        msg: String::from("Record created"),
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(success_msg),
        Err(err) => {
            let error_msg = ResponseMsg {
                msg: err.to_string(),
            };
            HttpResponse::InternalServerError().json(error_msg)
        }
    }
}

#[put("/records/update/{id}")]
pub async fn update_record(
    db: Data<Database>,
    id: Path<String>,
    new_record: Json<Record>,
) -> HttpResponse {
    let records_coll = db.collection::<Record>("records");

    let obj_id = match ObjectId::parse_str(id.into_inner()) {
        Ok(objid) => objid,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

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

    let obj_id = match ObjectId::parse_str(id.into_inner()) {
        Ok(objid) => objid,
        Err(err) => return HttpResponse::BadRequest().body(err.to_string()),
    };

    let filter = doc! {"_id": obj_id};

    let result = records_coll.delete_one(filter, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Deleted record"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
