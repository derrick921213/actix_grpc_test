use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix Web!")
}

pub fn rest_service() -> actix_web::Scope {
    actix_web::web::scope("").service(index)
}
