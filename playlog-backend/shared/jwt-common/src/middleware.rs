use super::{
    decode_token, extract_bearer_token, AuthClaims, JwtConfig,
    JwtError,
    Result, Role::{self, *},
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

/// Axum middleware that validates JWT tokens and extracts claims
///
/// This middleware:
/// - Extracts the Bearer token from the Authorization header
/// - Validates the token using the public key
/// - Inserts [AuthClaims] into request extensions for downstream handlers
///
pub async fn auth(
    State(config): State<JwtConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    let token = extract_bearer_token(req.headers())
        .map_err(|err| JwtError::InvalidAuthorizationHeader(err.to_string()))?;

    let claims = decode_token(&token, &config.public_key)?;

    let auth_claims = AuthClaims::try_from(claims)?;

    req.extensions_mut().insert(auth_claims);

    Ok(next.run(req).await)
}

/// Allows for extracting credentials for open routes if the user is authenticated
pub async fn auth_optional(
    State(config): State<JwtConfig>,
    mut req: Request,
    next: Next,
) -> Response {
    if let Ok(token) = extract_bearer_token(req.headers()) {
        if let Ok(claims) = decode_token(&token, &config.public_key) {
            if let Ok(auth_claims) = AuthClaims::try_from(claims) {
                req.extensions_mut().insert(auth_claims);
            }
        }
    }
    next.run(req).await
}

/// Middleware that allows any authenticated user ([User], [Moderator], or [Admin])
pub async fn require_user(req: Request, next: Next) -> Result<Response> {
    require_roles(req, next, &[User, Moderator, Admin]).await
}

/// Middleware that requires the user to be a [Moderator] or [Admin]
pub async fn require_moderator(req: Request, next: Next) -> Result<Response> {
    require_roles(req, next, &[Moderator, Admin]).await
}

/// Middleware that requires the user to be an [Admin]
pub async fn require_admin(req: Request, next: Next) -> Result<Response> {
    require_roles(req, next, &[Admin]).await
}

/// Role-based authorization guard
///
/// This middleware checks if the authenticated user has one of the required roles.
/// It **must** be used after [auth] middleware since it relies on [AuthClaims] being
/// present in request extensions.
///
/// # Usage
/// Use one of the helper functions [require_user], [require_moderator], [require_admin]
///
async fn require_roles(req: Request, next: Next, allowed_roles: &[Role]) -> Result<Response> {
    let claims = req
        .extensions()
        .get::<AuthClaims>()
        .ok_or(JwtError::MissingClaims)?;

    if !allowed_roles.contains(&claims.role) {
        Err(JwtError::Forbidden)
    } else {
        Ok(next.run(req).await)
    }
}
