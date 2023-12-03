use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{async_trait, http};

#[derive(Debug)]
pub struct UserIdentifier(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for UserIdentifier
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(
        req_parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    #[tokio::test]
    async fn test_user_identifier_extractor() {
        // given
        let user_id_value = "some-user-id";
        let mock_state = {};

        let request = Request::builder()
            .header("x-user-identifier", user_id_value)
            .body(Body::empty())
            .unwrap();
        let mut req_parts = request.into_parts().0;

        // when
        let extracted = UserIdentifier::from_request_parts(&mut req_parts, &mock_state)
            .await
            .unwrap();

        // then
        assert_eq!(extracted.0, user_id_value);
    }

    #[tokio::test]
    async fn test_user_identifier_extractor_missing_header() {
        // given
        let mock_state = {};
        let request = Request::builder().body(Body::empty()).unwrap();
        let mut req_parts = request.into_parts().0;

        // when
        let result = UserIdentifier::from_request_parts(&mut req_parts, &mock_state).await;

        // then
        assert!(matches!(result, Err((StatusCode::BAD_REQUEST, _))));
    }
}
