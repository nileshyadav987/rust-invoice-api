use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{bson::doc, options::ClientOptions, Client as MongoClient};
use serde::{Deserialize, Serialize};
use mongodb::bson::Bson;

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Invoice {
    invoice_id: String,
    amount: f64,
    status: String,
}

#[derive(Serialize, Deserialize)]
struct Client {
    client_id: String,
    name: String,
    email: String,
}

struct AppState {
    mongo_client: MongoClient,
    db_name: String,
}

#[post("/add")]
async fn add_item(data: web::Data<AppState>, item: web::Json<Item>) -> impl Responder {
    let collection = data.mongo_client.database(&data.db_name).collection("items");
    let doc = doc! { "name": &item.name };

    match collection.insert_one(doc, None).await {
        Ok(_) => HttpResponse::Ok().body("Item added successfully!"),
        Err(err) => {
            eprintln!("Failed to insert item: {}", err);
            HttpResponse::InternalServerError().body("Failed to add item")
        }
    }
}
// curl --location 'http://localhost:3000/add' \
// --header 'Content-Type: application/json' \
// --data-raw '{
//     "name": "nileshyadav987@gmail.com"
// }'

#[post("/invoice/add")]
async fn add_invoice(data: web::Data<AppState>, invoice: web::Json<Invoice>) -> impl Responder {
    let collection = data.mongo_client.database(&data.db_name).collection("invoices");
    let doc = doc! {
        "invoice_id": &invoice.invoice_id,
        "amount": &invoice.amount,
        "status": &invoice.status,
    };

    match collection.insert_one(doc, None).await {
        Ok(_) => HttpResponse::Ok().body("Invoice added successfully!"),
        Err(err) => {
            eprintln!("Failed to insert invoice: {}", err);
            HttpResponse::InternalServerError().body("Failed to add invoice")
        }
    }
}
// curl --location 'http://localhost:3000/invoice/add' \
// --header 'Content-Type: application/json' \
// --data '{
//         "invoice_id": "2",
//         "amount": 5000,
//         "status": "draft"
//     }'

#[put("/invoice/update/{invoice_id}")]
async fn update_invoice(data: web::Data<AppState>, invoice_id: web::Path<String>, invoice: web::Json<Invoice>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("invoices");
    let filter = doc! { "invoice_id": Bson::String(invoice_id.into_inner()) };
    let update = doc! {
        "$set": {
            "amount": &invoice.amount,
            "status": &invoice.status,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().body("Invoice updated successfully!"),
        Ok(_) => HttpResponse::NotFound().body("Invoice not found"),
        Err(err) => {
            eprintln!("Failed to update invoice: {}", err);
            HttpResponse::InternalServerError().body("Failed to update invoice")
        }
    }
}

#[delete("/invoice/delete/{invoice_id}")]
async fn delete_invoice(data: web::Data<AppState>, invoice_id: web::Path<String>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("invoices");
    let filter = doc! { "invoice_id": Bson::String(invoice_id.into_inner()) };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("Invoice deleted successfully!"),
        Ok(_) => HttpResponse::NotFound().body("Invoice not found"),
        Err(err) => {
            eprintln!("Failed to delete invoice: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete invoice")
        }
    }
}

#[get("/invoice/{invoice_id}")]
async fn get_invoice(data: web::Data<AppState>, invoice_id: web::Path<String>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("invoices");
    let filter = doc! { "invoice_id": Bson::String(invoice_id.into_inner()) };

    match collection.find_one(filter, None).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().body("Invoice not found"),
        Err(err) => {
            eprintln!("Failed to retrieve invoice: {}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve invoice")
        }
    }
}

#[post("/client/add")]
async fn add_client(data: web::Data<AppState>, client: web::Json<Client>) -> impl Responder {
    let collection = data.mongo_client.database(&data.db_name).collection("clients");
    let doc = doc! {
        "client_id": &client.client_id,
        "name": &client.name,
        "email": &client.email,
    };

    match collection.insert_one(doc, None).await {
        Ok(_) => HttpResponse::Ok().body("Client added successfully!"),
        Err(err) => {
            eprintln!("Failed to insert client: {}", err);
            HttpResponse::InternalServerError().body("Failed to add client")
        }
    }
}

#[put("/client/update/{client_id}")]
async fn update_client(data: web::Data<AppState>, client_id: web::Path<String>, client: web::Json<Client>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("clients");
    let filter = doc! { "client_id": Bson::String(client_id.into_inner()) };
    let update = doc! {
        "$set": {
            "name": &client.name,
            "email": &client.email,
        }
    };

    match collection.update_one(filter, update, None).await {
        Ok(result) if result.matched_count > 0 => HttpResponse::Ok().body("Client updated successfully!"),
        Ok(_) => HttpResponse::NotFound().body("Client not found"),
        Err(err) => {
            eprintln!("Failed to update client: {}", err);
            HttpResponse::InternalServerError().body("Failed to update client")
        }
    }
}

#[delete("/client/delete/{client_id}")]
async fn delete_client(data: web::Data<AppState>, client_id: web::Path<String>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("clients");
    let filter = doc! { "client_id": Bson::String(client_id.into_inner()) };

    match collection.delete_one(filter, None).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().body("Client deleted successfully!"),
        Ok(_) => HttpResponse::NotFound().body("Client not found"),
        Err(err) => {
            eprintln!("Failed to delete client: {}", err);
            HttpResponse::InternalServerError().body("Failed to delete client")
        }
    }
}

#[get("/client/{client_id}")]
async fn get_client(data: web::Data<AppState>, client_id: web::Path<String>) -> impl Responder {
    let collection: mongodb::Collection<mongodb::bson::Document> = data.mongo_client.database(&data.db_name).collection("clients");
    let filter = doc! { "client_id": Bson::String(client_id.into_inner()) };

    match collection.find_one(filter, None).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().body("Client not found"),
        Err(err) => {
            eprintln!("Failed to retrieve client: {}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve client")
        }
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // MongoDB Connection
    let mongo_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let mongo_client = MongoClient::with_options(client_options).unwrap();
    let db_name = "invoice-demo-rust".to_string(); // You can customize this.

    // Shared application state
    let app_state = web::Data::new(AppState { mongo_client, db_name });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Use shared app state
            .service(hello)
            .service(echo)
            .service(add_item)
            .service(add_invoice)
            .service(update_invoice)
            .service(delete_invoice)
            .service(get_invoice)
            .service(add_client)
            .service(update_client)
            .service(delete_client)
            .service(get_client)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("localhost", 3000))?
    .run()
    .await
}
