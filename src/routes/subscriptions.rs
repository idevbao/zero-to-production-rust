use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::{PgPool};
use tracing::Instrument;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name= %form.name
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    let pg_query_result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await;

    match pg_query_result {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );

            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
    /*We use `get_ref` to get an immutable reference to the `PgConnection`
    wrapped by `web::Data`.
    sqlx has an asynchronous interface,
    but it does not allow you to run multiple queries concurrently over the same database connection.
    thay vi su dung Mutex de gianh quyen truy cap ket noi -> han che 1 truy van tai 1 thoi diem
    khóa mut, nó có nghĩa là biến đó là biến có tính thay đổi và bạn có thể thay đổi giá trị của nó trong quá trình chạy chương trình.

    Nếu bạn sử dụng biến tham chiếu có tính thay đổi này để truy cập vào cơ sở dữ liệu,
    bạn nên đảm bảo rằng chỉ có một luồng hoặc tiến trình có thể truy cập vào kết nối cơ sở dữ liệu tại một thời điểm.
    but of a different kind: when you run a query against a &PgPool,
    sqlx will borrow a PgConnection from the pool and use it to execute the query;
    */
}
