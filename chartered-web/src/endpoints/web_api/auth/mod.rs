use axum::{
    body::{Body, BoxBody},
    extract,
    handler::{get, post},
    http::{Request, Response},
    Router,
};
use chartered_db::{
    users::{User, UserSession},
    uuid::Uuid,
    ConnectionPool,
};
use futures::future::Future;
use serde::Serialize;
use std::convert::Infallible;

pub mod logout;
pub mod openid;
pub mod password;

pub fn authenticated_routes() -> Router<
    impl tower::Service<
            Request<Body>,
            Response = Response<BoxBody>,
            Error = Infallible,
            Future = impl Future<Output = Result<Response<BoxBody>, Infallible>> + Send,
        > + Clone
        + Send,
> {
    crate::axum_box_after_every_route!(Router::new().route("/logout", get(logout::handle)))
}

pub fn unauthenticated_routes() -> Router<
    impl tower::Service<
            Request<Body>,
            Response = Response<BoxBody>,
            Error = Infallible,
            Future = impl Future<Output = Result<Response<BoxBody>, Infallible>> + Send,
        > + Clone
        + Send,
> {
    crate::axum_box_after_every_route!(Router::new()
        .route("/login/password", post(password::handle))
        .route("/login/oauth/:provider/begin", get(openid::begin_oidc))
        .route("/login/oauth/complete", get(openid::complete_oidc))
        .route("/login/oauth/providers", get(openid::list_providers)))
}

#[derive(Serialize)]
pub struct LoginResponse {
    user_uuid: Uuid,
    key: String,
    expires: chrono::DateTime<chrono::Utc>,
    picture_url: Option<String>,
}

/// Takes the given `User` and generates a session for it and returns a response containing an API
/// key to the frontend that it can save for further request
pub async fn login(
    db: ConnectionPool,
    user: User,
    user_agent: Option<extract::TypedHeader<headers::UserAgent>>,
    extract::ConnectInfo(addr): extract::ConnectInfo<std::net::SocketAddr>,
) -> Result<LoginResponse, chartered_db::Error> {
    let user_agent = if let Some(extract::TypedHeader(user_agent)) = user_agent {
        Some(user_agent.as_str().to_string())
    } else {
        None
    };

    let expires = chrono::Utc::now() + chrono::Duration::hours(1);
    let key = UserSession::generate(
        db,
        user.id,
        None,
        Some(expires.naive_utc()),
        user_agent,
        Some(addr.to_string()),
    )
    .await?;

    Ok(LoginResponse {
        user_uuid: user.uuid.0,
        key: key.session_key,
        expires,
        picture_url: user.picture_url,
    })
}
