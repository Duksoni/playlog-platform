use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use crate::{
    model::{GameLibraryStatus, UserGame},
    dto::AddUpdateGameRequest
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "Library Service"),
    paths(
        crate::handler::get_user_library,
        crate::handler::get_library_stats,
        crate::handler::add_or_update_game,
        crate::handler::remove_from_library,
    ),
    components(
        schemas(GameLibraryStatus, UserGame, AddUpdateGameRequest)
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );
        }
    }
}
