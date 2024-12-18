use core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};

use actix_web::{HttpRequest, HttpResponse};
use glue::errors::NanoServiceError;

pub async fn delete_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    match req.match_info().get("name") {
        Some(name) => {
            delete_core(name).await?;
        }
        None => {
            return Err(NanoServiceError::new(
                "Name not provided".to_string(),
                glue::errors::NanoServiceErrorStatus::BadRequest,
            ))
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
