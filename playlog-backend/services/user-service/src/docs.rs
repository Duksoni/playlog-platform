use service_common::docs::SecurityAddon;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(title = "User Service"),
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
