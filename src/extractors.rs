use axum::{
    async_trait,
    http,
};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Debug)]
pub struct UserIdentifier(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for UserIdentifier
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id_header = req_parts.headers.get("x-user-identifier");

        match user_id_header {
            Some(value) => Ok(UserIdentifier(value.to_str().unwrap().to_string())),
            None => Err((
                http::StatusCode::BAD_REQUEST,
                "Missing X-User-Identifier header",
            )),
        }
    }
}
