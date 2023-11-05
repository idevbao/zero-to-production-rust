use actix_web::{web, App, HttpServer, HttpResponse, Result};
use futures::StreamExt;

pub async fn upload_data(mut body: web::Payload) -> Result<HttpResponse> {
    let mut bytes = Vec::new();
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk);
    }

    // Xử lý dữ liệu ở đây, ví dụ: lưu trữ vào một tệp
    let result = std::fs::write("uploaded_data.txt", bytes);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Dữ liệu đã được tải lên thành công!")),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
