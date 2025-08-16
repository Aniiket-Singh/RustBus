use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{http::StatusCode, Error, FromRequest, Result};

use crate::routes::user::Claims;

pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId{
    async fn from_request(
            req: &'a poem::Request,
            _body: &mut poem::RequestBody,
        ) -> Result<Self> {
            let token = req
                .headers()
                .get("authorization")
                .and_then(|value|value.to_str().ok())
                .ok_or_else(|| Error::from_string("missing token",
                StatusCode::UNAUTHORIZED))?;

            let token_data = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()).map_err(|_| Error::from_string("token malformed", StatusCode::UNAUTHORIZED))?;
            Ok(UserId(token_data.claims.sub))
    }
}