use actix_web::{web::Json, HttpResponse};
use core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use core::structs::ToDoItem;
use glue::errors::NanoServiceError;

pub async fn create(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
