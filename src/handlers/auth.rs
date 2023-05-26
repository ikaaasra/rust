use crate::{
    model::UserModel,
    schema::{GenericResponse, Signin, Signup, UserSingleResponse, JWT},
    AppState,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json,
};

use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use std::sync::Arc;

// ----------------------------------------------------------------- SIGNUP_TODO
pub async fn signup_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Signup>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_exists: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE mail = $1)")
            .bind(body.mail.to_owned().to_ascii_lowercase())
            .fetch_one(&data.db)
            .await
            .map_err(|e| {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": format!("Database error: {}", e),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

    if let Some(exists) = user_exists {
        if exists {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "User with that mail already exists",
            });
            return Err((StatusCode::CONFLICT, Json(error_response)));
        }
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|err| {
            let response = serde_json::json!(GenericResponse {
                status: "fail".to_string(),
                message: format!("Error while hashing password: {}", err),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        })
        .map(|hash| hash.to_string())?;

    let query = sqlx::query_as!(
        UserModel,
        "INSERT INTO users (name,mail,password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.mail.to_string().to_ascii_lowercase(),
        hashed_password,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| {
        let response = serde_json::json!(GenericResponse {
            status: "fail".to_string(),
            message: format!("Database error: {}", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
    })?;

    let response = serde_json::json!(UserSingleResponse {
        status: "success".to_string(),
        data: query
    });

    return Ok((StatusCode::OK, Json(response)));
}

// ----------------------------------------------------------------- SIGNIN_TODO
pub async fn signin_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<Signin>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE mail = $1",
        body.mail.to_ascii_lowercase(),
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|err| {
        let response = serde_json::json!(GenericResponse {
            status: "error".to_string(),
            message: format!("Database error: {}", err),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
    })?
    .ok_or_else(|| {
        let response = serde_json::json!(GenericResponse {
            status: "fail".to_string(),
            message: "Invalid mail or password".to_string(),
        });
        (StatusCode::BAD_REQUEST, Json(response))
    })?;

    let valid = match PasswordHash::new(&query.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(GenericResponse {
                status: "fail".to_string(),
                message: "Invalid mail or password".to_string()
            })),
        ));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: JWT = JWT {
        sub: query.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response =
        Response::new(serde_json::json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)

    // return Ok((StatusCode::OK, Json({})));
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut response = Response::new(serde_json::json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<UserModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(serde_json::json!(UserSingleResponse {
        status: "success".to_string(),
        data: user
    })))
}
