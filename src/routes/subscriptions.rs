use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String
}


pub async fn subscriptions(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}