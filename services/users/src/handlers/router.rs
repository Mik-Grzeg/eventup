use crate::{
    app_state::AppState,
    middlewares::{self, auth::RequireAuth},
};
use axum::{
    middleware::{from_extractor, from_extractor_with_state},
    routing::{delete, get, post},
    Router,
};

use tower_http::trace::TraceLayer;

use super::{delete_user, get_user, login, access_control, post_user, put_user};

pub fn router(app_state: AppState) -> Router {
    let health_route = Router::new().route("/health", get(super::health::health));

    let user_routers = Router::new()
        .route(
            "/:id",
            delete(delete_user::delete_user)
                .put(put_user::update_user)
                .get(get_user::get_user),
        )
        .route_layer(from_extractor_with_state::<RequireAuth, AppState>(
            app_state.clone(),
        ))
        .route("/", post(post_user::create_user))
        .with_state(app_state.clone());

    let auth_routers = Router::new()
        .route("/login", post(login::login))
        .route("/access", get(access_control::access_control))
        .with_state(app_state);

    let api_routes = Router::new()
        .nest("/users", user_routers)
        .nest("/auth", auth_routers);

    Router::new()
        .nest("/api/v1", api_routes)
        .merge(health_route)
        .layer(TraceLayer::new_for_http())
}