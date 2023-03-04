pub mod error {
    use std::convert::Infallible;
    use std::str::Utf8Error;
    use std::string::FromUtf8Error;
    use axum::{http, http::StatusCode, Json, response::{IntoResponse, Response}};
    use axum::extract::rejection::JsonRejection;
    use axum::http::header::ToStrError;
    use serde_json::json;

    #[derive(thiserror::Error, Debug)]
    #[non_exhaustive]
    pub enum ApiError {

        #[error("{0}")]
        BadRequestError(String),

        #[error("runtime error")]
        RuntimeErr(ToStrError),
    }

    impl IntoResponse for ApiError {
        fn into_response(self) -> axum::response::Response {

            let (code, message) = match self {
                ApiError::BadRequestError(error_msg) => {
                    (http::StatusCode::BAD_REQUEST.as_u16(), error_msg)
                }
                ApiError::RuntimeErr(to_str_error) => {
                    (http::StatusCode::BAD_REQUEST.as_u16(), to_str_error.to_string())
                }
            };

            let payload = json!({
                "message": message,
                "code": code
            });

            Ok::<Json<serde_json::Value>, Infallible>(Json(payload)).into_response()
        }
    }

    // impl From<Utf8Error> for ApiError {
    //     fn from(error: Utf8Error) -> Self {
    //         ApiError::RuntimeErr(error)
    //     }
    // }

    impl From<ToStrError> for ApiError {
        fn from(error: ToStrError) -> Self {
            ApiError::RuntimeErr(error)
        }
    }

}
