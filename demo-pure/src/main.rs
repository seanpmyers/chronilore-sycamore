use std::net::SocketAddr;

use axum::{
    routing::{get_service, MethodRouter},
    Router,
};
use tower_http::services::fs::ServeDir;

const FRONTEND_DIST_PATH: &str = "../sycamore9/dist";

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let directory_service: ServeDir = ServeDir::new(FRONTEND_DIST_PATH);
    let front_end_directory_router: MethodRouter = get_service(directory_service);

    let application_router: Router = Router::new().fallback_service(front_end_directory_router);

    let socket_address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 4100_u16));
    axum_server::bind(socket_address)
        .serve(application_router.into_make_service())
        .await
        .unwrap();
}
