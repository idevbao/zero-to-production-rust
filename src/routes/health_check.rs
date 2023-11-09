use actix_web::{HttpResponse};

pub async fn health_check() -> HttpResponse {
    // let ten_millis = time::Duration::from_secs(10);
    // let now = time::Instant::now();
    // thread::sleep(ten_millis);
    HttpResponse::Ok().body("OK Http Response!")
}
