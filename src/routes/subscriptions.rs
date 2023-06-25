use crate::FormData;
use actix_web::{web, HttpResponse, Responder};

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
