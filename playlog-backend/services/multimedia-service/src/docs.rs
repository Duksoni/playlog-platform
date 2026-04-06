use service_common::docs::SecurityAddon;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "Multimedia Service"),
    paths(
        crate::handler::get_game_covers,
        crate::handler::get_game_media,
        crate::handler::upload_game_media,
        crate::handler::delete_game_media,
        crate::app::health_check,
    )
)]
pub struct ApiDoc;
