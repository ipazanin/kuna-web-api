mod config;
mod dtos;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use routes::users::create_user;
use std::{
    net::{SocketAddr, SocketAddrV4},
    str::FromStr,
};

#[tokio::main]
async fn main() {
    rust_test();

    let configuration = config::create_config::create_config();
    let port = configuration.server_config().port();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(routes::root::hello_world))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    let address = format!("0.0.0.0:{port}");

    // run our app with hyper, listening globally on port 3000
    let socket_address =
        SocketAddr::V4(SocketAddrV4::from_str(&address).expect("Invalid Socket Address"));

    axum::Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn rust_test() {
    println!("Test rust function");
}
