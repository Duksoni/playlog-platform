use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "Catalogue Service"),
    paths(
        crate::app::health_check,
        crate::games::handler::filter,
        crate::games::handler::find_by_developer,
        crate::games::handler::find_by_publisher,
        crate::games::handler::get_detail,
        crate::games::handler::get_unpublished,
        crate::games::handler::create,
        crate::games::handler::update,
        crate::games::handler::delete_game,
        crate::games::handler::publish,
        crate::games::handler::unpublish,
        crate::developers::handler::get_all_paged,
        crate::developers::handler::get_by_id,
        crate::developers::handler::search,
        crate::developers::handler::create,
        crate::developers::handler::update,
        crate::genres::handler::get_all,
        crate::genres::handler::get_by_id,
        crate::genres::handler::search,
        crate::genres::handler::create,
        crate::genres::handler::update,
        crate::platforms::handler::get_all,
        crate::platforms::handler::get_by_id,
        crate::platforms::handler::search,
        crate::platforms::handler::create,
        crate::platforms::handler::update,
        crate::publishers::handler::get_all_paged,
        crate::publishers::handler::get_by_id,
        crate::publishers::handler::search,
        crate::publishers::handler::create,
        crate::publishers::handler::update,
        crate::tags::handler::get_all_paged,
        crate::tags::handler::get_by_id,
        crate::tags::handler::search,
        crate::tags::handler::create,
        crate::tags::handler::update,
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
