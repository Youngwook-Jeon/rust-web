use actix_web::web::{get, post, scope, ServiceConfig};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("get/all", get().to(get::get_all))
            .route("get/{name}", get().to(get::get_by_name))
            .route("create", post().to(create::create)),
    );
}
