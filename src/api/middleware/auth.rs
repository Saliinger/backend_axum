use axum::extract::FromRequestParts;
use axum::http::{StatusCode, header::AUTHORIZATION, request::Parts};
// use cookie::Cookie;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NextAuthClaims {
    // NextAuth commonly puts the user id in `sub` or in custom `id` claim.
    pub sub: Option<String>,
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub exp: usize,
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1) try Authorization header
        let token_opt = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_string()));

        // 2) fallback to cookie
        let token = match token_opt {
            Some(t) => t,
            // None => {
            //     if let Some(cookie_hdr) = parts.headers.get("cookie") {
            //         if let Ok(cookie_str) = cookie_hdr.to_str() {
            //             // NextAuth uses these cookie names in prod/dev
            //             for name in &[
            //                 "__Secure-next-auth.session-token",
            //                 "next-auth.session-token",
            //             ] {
            //                 if let Some(cookie) = cookie_str
            //                     .split(';')
            //                     .map(|c| c.trim())
            //                     .find(|c| c.starts_with(&format!("{}=", name)))
            //                 {
            //                     if let Ok(parsed) = Cookie::parse_encoded(cookie) {
            //                         let val = parsed.value().to_string();
            //                         if !val.is_empty() {
            //                             token_opt.replace(val);
            //                             break;
            //                         }
            //                     }
            //                 }
            //             }
            //         }
            //     }
            //     match token_opt {
            //         Some(t) => t,
            None => return Err(StatusCode::UNAUTHORIZED),
            // }
            // }
        };

        // 3) verify using NEXTAUTH_SECRET env var
        let secret =
            std::env::var("NEXTAUTH_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<NextAuthClaims>(&token, &key, &validation)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let claims = token_data.claims;
        let id = claims.id.or(claims.sub).ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            id,
            email: claims.email,
            name: claims.name,
        })
    }
}
