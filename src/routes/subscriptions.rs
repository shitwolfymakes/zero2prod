//! src/routs/subscriptions.rs
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,                        // % tells `tracing` to use their Display impl for logging purposes
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );
    // using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't do this at home.
    // See the following section on `Instrumenting Futures`
    let _request_span_guard = request_span.enter();

    // we do not call `.enter` on query_span!
    // `.instrument` takes care of the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    // first we attach the instrumentation, then we await it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
    // `_request_span_guard` is dropped at the end of `subscribe`
    // that's where we "exit" the span
}