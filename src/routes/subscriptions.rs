use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
name = "Adding a new subscriber",
skip(form, pool),
fields(
    subscriber_email = %form.email, subscriber_name= %form.name
))]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    if !(is_valid_name(&form.name)) || !(is_valid_name(&form.email)) {
        return HttpResponse::BadRequest().finish();
    }

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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
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

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
"#,
        Uuid::new_v4(),
        form.email,
        form.name,
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

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set, // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    // Iterate over all characters in the input `s` to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    // Return `false` if any of our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
