pub mod resp {
    use std::convert::Infallible;
    use axum::{http, Json};
    use axum::http::Response;
    use axum::response::IntoResponse;
    use hyper::Body;
    use serde::ser::{Serialize, SerializeStruct, Serializer};
    use serde_json::json;
    use crate::error::error::ApiError;

    #[derive(Debug)]
    pub(crate) struct IResponse<T> {
        code: u16,
        message: &'static str,
        data: Option<T>
    }

    impl <T> IResponse<T>
        where T: serde::Serialize
    {
        pub fn new(code: u16, message: &'static str, data: Option<T>) -> IResponse<T> {
            IResponse {
                code,
                message,
                data,
            }
        }

        pub fn ok(data: Option<T>) -> IResponse<T> {
            IResponse {
                code: http::StatusCode::OK.as_u16(),
                message: "Ok",
                data,
            }
        }

        pub fn bad_request(message: &'static str) -> IResponse<T> {
            IResponse {
                code: http::StatusCode::BAD_REQUEST.as_u16(),
                message,
                data: None,
            }
        }

        // pub fn to_axum_resp(self) -> Result<Response<Body>, anyhow::Error> {
        //     Ok::<_, Infallible>(Response::new(body))
        // }
    }

    impl <T> IntoResponse for IResponse<T>
        where T: serde::Serialize
    {
        fn into_response(self) -> axum::response::Response {
            let payload = json!({
                "message": self.message,
                "code": self.code,
                "data": self.data
            });

            Ok::<Json<serde_json::Value>, Infallible>(Json(payload)).into_response()
        }
    }


    // impl <T> From<ApiError> for IResponse<T>{
    //     fn from(value: ApiError) -> Self {
    //         todo!()
    //     }
    // }

    // impl <T> Serialize for IResponse<T> {
    //     fn serialize<S>(&self, serializer: S) -> Result<serde::ser::Ok, dyn serde::ser::Error>
    //         where S: Serializer
    //     {
    //         let mut s = serializer.serialize_struct("IResponse", 3)?;
    //         s.serialize_field("code", &self.code)?;
    //         s.serialize_field("message", "Ok")?;
    //         s.serialize_field("data", &self.data)?;
    //         s.end()
    //     }
    // }

}
