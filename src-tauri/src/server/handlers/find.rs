use actix_web::{get, HttpResponse, Responder};

#[get("/find")]
async fn handle() -> impl Responder {
    HttpResponse::Ok().body("doppio_server")
}
