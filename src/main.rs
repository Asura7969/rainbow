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
use std::borrow::BorrowMut;
use std::convert::Infallible;
use std::error::Error;
use axum::http::uri::PathAndQuery;
use serde_json::{json, Value};
use tower_http::{classify::ServerErrorsFailureClass,
                 trace::TraceLayer};

use tower::{service_fn, ServiceBuilder};
use tower::{make::Shared, ServiceExt};
use tracing::{info, warn, error, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


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

    let service = tower::service_fn(move |req: Request<Body>| {
        // let router_svc = app.clone();
        async move {

            for (h_name, h_value) in req.headers() {
                println!("header name: {:?}, header value: {:?}", h_name, h_value);
            }
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
            let res = Response::new(Body::from("Hi from `GET /`"));
            Ok::<_, Infallible>(res)
            // if req.method() == Method::CONNECT {
            //     proxy(req).await
            // } else {
            //     router_svc.oneshot(req).await.map_err(|err| match err {})
            // }
        }
    });

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(Shared::new(service))
        .await
        .unwrap();
}


