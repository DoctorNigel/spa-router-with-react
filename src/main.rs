use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use axum_extra::routing::RouterExt;
use std::{io, net::SocketAddr};
use axum::http::{Request, Response, Uri};
use tower_http::{services::ServeDir, trace::TraceLayer, services::ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // `SpaRouter` is the easiest way to serve assets at a nested route like `/assets`
    let app = Router::new()
        .route("/", get_service(ServeDir::new("./static/index.html")).handle_error(handle_error))
        .merge(axum_extra::routing::SpaRouter::new("/static", "./static/static").index_file("./static/index.html"))
        .layer(TraceLayer::new_for_http());

    /*
    let app: _ = Router::new()
        .route("/", get_service(ServeDir::new("./static/index.html")).handle_error(handle_error))
        .route("/static", get_service(ServeDir::new("./static/static")).handle_error(handle_error))
        .route("/chromacells", get_service(ServeDir::new("./static/static")).handle_error(handle_error))

        .fallback_service(get_service(ServeDir::new("./static/")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

     */

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}


/*
 let app = Router::new()
        .route("/", get_service(ServeDir::new("./static/index.html")).handle_error(handle_error))
         .merge(axum_extra::routing::SpaRouter::new("/static", "./static/"))
         .layer(TraceLayer::new_for_http());
 */