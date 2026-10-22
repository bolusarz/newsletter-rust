use actix_web::{HttpResponse, Responder, post, web};

#[derive(serde::Deserialize)]
struct SubscriptionPayload {
    email: String,
    name: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<SubscriptionPayload>) -> impl Responder {
    HttpResponse::Ok().finish()
}
