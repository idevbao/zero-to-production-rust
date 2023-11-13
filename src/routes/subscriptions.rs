use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
    subscriber_email = %form.email, subscriber_name= %form.name
))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let name = match SubscriberName::parse(form.0.name) {
        Ok(_name) => _name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(_email) => _email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let new_subscriber = NewSubscriber { email, name };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
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

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error // We will talk about error handling in depth later!
    })?;
    Ok(())
}
