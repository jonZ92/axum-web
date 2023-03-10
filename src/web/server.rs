use std::{marker::PhantomData, net::SocketAddr};

use axum::{
    routing::{get, post},
    Router,
};


use tower_http::auth::RequireAuthorizationLayer;

use crate::web;

pub async fn server() {
    tracing_subscriber::fmt::init();



    let login = Router::new().route("/login", post(web::login::get_token));

    let app = Router::new()
        .route("/", get(web::jsons::root))
        .route("/users", post(web::jsons::create_user))
        //  请求参数带有内容
        .route("/getPath/:id", get(web::paths::get_user))
        .route("/upload", post(web::upload::upload))
        .route("/query/user", get(web::database::test_mysql::query_user))
        .route_layer(RequireAuthorizationLayer::custom(
            web::security::Authorization { _ty: PhantomData },
        ));

    let all = Router::new().merge(login).merge(app);

    let addr = SocketAddr::from(web::configuration::server_configuration());
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(all.into_make_service())
        .await
        .unwrap();
}
