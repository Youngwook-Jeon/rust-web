use actix_web::{web::Json, HttpResponse};
use core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use core::structs::ToDoItem;
use glue::errors::NanoServiceError;

pub async fn update(body: Json<ToDoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
