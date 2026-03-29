use service_common::docs::SecurityAddon;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "Library Service"),
    paths(
        crate::handler::get_user_library,
        crate::handler::get_library_stats,
        crate::handler::add_or_update_game,
        crate::handler::remove_from_library,
    )
)]
pub struct ApiDoc;
