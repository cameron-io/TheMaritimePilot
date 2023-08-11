use axum::http::StatusCode;

// Utility function for mapping any error into a `500 Internal Server Error` response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

// use std::env;
// pub fn getenv(key: String) -> Result<String, env::VarError> {
//     let result = env::var(key.clone());
//     match result {
//         Ok(value) => {
//             return Ok(value, );
//         },
//         Err(error) => {
//             tracing::debug!("failed to resolve environment variable: {}", key);
//             return Err(error);
//         }
//     }
// }
