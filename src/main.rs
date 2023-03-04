mod error;
mod resp;

use axum::{
    body::{Bytes, Body},
    http::{Request, HeaderMap, StatusCode, Method, Uri},
    error_handling::{HandleError, HandleErrorLayer},
    response::{Html, Response, IntoResponse},
    routing::{get, post, any_service}, Json, Router, BoxError,
    extract::{Extension, Path}
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};
use std::any::Any;
use std::borrow::BorrowMut;
use std::convert::Infallible;
use std::error::Error;
use axum::http::uri::PathAndQuery;
use hyper::http;
use serde_json::{json, Value};
use tower_http::{classify::ServerErrorsFailureClass,
                 trace::TraceLayer};

use tower::{service_fn, ServiceBuilder};
use tower::{make::Shared, ServiceExt};
use tracing::{info, warn, error, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tonic::client::Grpc;
use tonic::transport::{Channel, Endpoint};
use crate::error::error::ApiError;
use crate::error::error::ApiError::BadRequestError;
use crate::resp::resp::IResponse;

#[tokio::main]
async fn main() {

    // let pool = AppState::creat_db_pool("mysql://root:123456@localhost/skeleton", 5).await;
    // let redis = AppState::creat_redis("redis://127.0.0.1/").await;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rainbow=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    // let app = Router::new()
    //     .route("/create_user", post(create_user))
    //     .route("/user_by_id/:id", get(query_user_by_id));
    // let app = Router::new()
        // .route("/", get(usage))
        // .route("/err/:id", get(error_handler))
        // .route("/create_user", post(create_user))
        // .route("/user_by_id/:id", get(query_user_by_id))
        // .route("/user_by_name/:name", get(query_user_by_name))
        // .route(
        //     "/",
        //     // Services whose response body is not `axum::body::BoxBody`
        //     // can be wrapped in `axum::routing::any_service` (or one of the other routing filters)
        //     // to have the response body mapped
        //     any_service(service_fn(|req: Request<Body>| async move {
        //         // let header_map = body.headers();
        //         for (h_name, h_value) in req.headers() {
        //             println!("header name: {:?}, header value: {:?}", h_name, h_value);
        //         }
        //         let (mut parts, body) = req.into_parts();
        //
        //         let body_bytes = hyper::body::to_bytes(body).await.unwrap();
        //         println!("body: {:?}", body_bytes);
        //         let uri = parts.uri;
        //         if let Some(path_and_query) = uri.path_and_query() {
        //             println!("path: {}", path_and_query.path());
        //             if let Some(q) = path_and_query.query() {
        //                 println!("query: {}", q);
        //             }
        //         }
        //
        //         let res = Response::new(Body::from("Hi from `GET /`"));
        //         Ok::<_, Infallible>(res)
        //     }))
        // )
        // .layer(Extension(AppState { pool }))
        // .layer(
        //     TraceLayer::new_for_http()
        //         .on_request(|request: &Request<_>, _span: &Span| {
        //             info!("started {} {}", request.method(), request.uri().path())
        //         })
        //         .on_response(|_response: &Response, latency: Duration, _span: &Span| {
        //             info!("response generated in {:?}", latency)
        //         })
        //         .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
        //             info!("sending {} bytes", chunk.len())
        //         })
        //         .on_eos(
        //             |_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
        //                 warn!("stream closed after {:?}", stream_duration)
        //             },
        //         )
        //         .on_failure(
        //             |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
        //                 error!("something went wrong")
        //             },
        //         ),
        // );

    let service = service_fn(move |req: Request<Body>| {
        async move {
            if req.headers().get(http::header::CONTENT_TYPE)
                .filter(|application_json| {
                    application_json.to_str().map(|json| json.eq(mime::APPLICATION_JSON.as_ref())).is_ok()
                }).is_none() {

                let body = Json(json!({
                    "code": StatusCode::BAD_REQUEST.as_u16(),
                    "msg": "Header does not exist appid",
                    "data": ""
                }));

                Ok::<Response, Infallible>((StatusCode::BAD_REQUEST, body).into_response())
            } else {
                let appid = req.headers()
                    .get("appid")
                    .and_then(|v| v.to_str().ok());

                if appid.is_none() {
                    let body = Json(json!({
                        "code": StatusCode::BAD_REQUEST.as_u16(),
                        "msg": "Appid parsing failed or missing",
                        "data": ""
                    }));
                    Ok::<Response, Infallible>((StatusCode::BAD_REQUEST, body).into_response())
                } else {
                    println!("appid: {}", appid.unwrap());

                    // 1、获取appid 对应的路由信息
                    //    1.1 获取appid 元数据信息，是否鉴权、限流、协议转换等
                    // 2、解析url，获取 header、query、body，解析参数

                    let (mut parts, body) = req.into_parts();

                    let body_bytes = hyper::body::to_bytes(body).await.unwrap();
                    println!("body: {:?}", body_bytes);
                    let uri = parts.uri;
                    if let Some(path_and_query) = uri.path_and_query() {
                        println!("path: {}", path_and_query.path());
                        if let Some(q) = path_and_query.query() {
                            println!("query: {}", q);
                        }
                    }
                    let body = Json(json!({
                        "code": StatusCode::OK.as_u16(),
                        "msg": "Ok",
                        "data": ""
                    }));
                    Ok::<Response, Infallible>((StatusCode::OK, body).into_response())
                }
            }


        }
    });

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();
}





// async fn test_start() -> Result<(), Box<dyn Error>> {
//     // 服务地址与接口信息应从外部获取, 此处为测试
//     let addr = Endpoint::from_static("http://127.0.0.1:9091");
//     let mut client_grpc: Grpc<Channel> = connect(addr.clone()).await;
//     /// key: 服务名 + 接口 + 方法名, value: request json元数据
//     ///
//     /// proto 文件定义内容:
//     /// ```
//     /// message HelloRequest {
//     ///   string name = 1;
//     ///   int32 age = 2;
//     ///   map<string, string> tags = 3;
//     /// }
//     /// ```
//     /// request json元数据
//     ///
//     /// type: grpc类型, path: http请求中的参数路径(JsonPath)
//     /// ```
//     /// {
//     ///     "name": {"type": "string", "path": "$.path.name"},
//     ///     "age": {"type": "int32", "path": "$.query.age"},
//     ///     "tags": {
//     ///         "type": "map",
//     ///         "pathMap": {
//     ///             "key1": "$.body.desc",
//     ///             "key2": "$.body.orderId",
//     ///             "key3": "$.body.name",
//     ///         }
//     ///     }
//     /// }
//     /// ```
//     unary(&mut client_grpc, "/com.asura.grpc.HelloService/SayHello")
// }
//
// async fn unary<T>(client: &mut Grpc<Channel>,
//                   method: &'static str) -> Result<tonic::Response<T>, tonic::Status> {
//     client
//         .ready()
//         .await
//         .map_err(|e| {
//             tonic::Status::new(
//                 tonic::Code::Unknown,
//                 format!("Service was not ready: {}", e.into()),
//             )
//         })?;
//     let codec = tonic::codec::ProstCodec::default();
//     let path = http::uri::PathAndQuery::from_static(method);
//     client.unary(request.into_request(), path, codec).await
// }
//
//
// async fn connect<D>(dst: D) -> Grpc<Channel>
//     where
//         D: std::convert::TryInto<tonic::transport::Endpoint>,
//         D::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
// {
//     let channel = Endpoint::new(dst)?.connect().await?;
//     Grpc::new(channel)
// }
// fn response_body(message: &str, code: u16, data: Option<&dyn Any>) -> Response<Body> {
//     let payload = json!({
//         "message": message,
//         "code": code,
//         "data": data,
//     });
//     Response::new(Body::from(axum::Json::from(payload).as_str().unwrap()))
// }
