use crate::detail_post::DetailPost;
use actix_web::{HttpResponse, Result};

pub async fn get_md_list() -> Result<HttpResponse> {
    let detail_post: DetailPost = DetailPost {
        title: "get_md_list_title".to_string(),
    };
    let str = serde_json::to_string(&detail_post);
    match str {
        Ok(body) => Ok(HttpResponse::Ok().body(body)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
