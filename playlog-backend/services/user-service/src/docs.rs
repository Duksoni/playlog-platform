use utoipa::{
    Modify,
    OpenApi,
    openapi::{
        security::{Http, HttpAuthScheme, SecurityScheme},
        SecurityRequirement
    }
};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "User Service", description = "User service description"),
    paths(
        crate::auth::handler::login,
        crate::auth::handler::register,
        crate::auth::handler::logout,
        crate::auth::handler::refresh_tokens,
        crate::users::handler::get_user,
        crate::users::handler::update_user,
        crate::users::handler::change_password,
        crate::users::handler::deactivate_account,
        crate::users::handler::find_users,
        crate::users::handler::promote_user,
        crate::users::handler::demote_user,
        crate::users::handler::block_user,
        crate::app::health_check
    ),

)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
        openapi.security = Some(vec![SecurityRequirement::new("bearer", Vec::<&str>::new())]);
    }
}