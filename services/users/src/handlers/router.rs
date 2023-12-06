use crate::{app_state::AppState, middlewares::auth::Authorization};
use axum::{
    middleware::from_extractor_with_state,
    routing::{delete, get, post},
    Router,
};

use tower_http::trace::TraceLayer;

use super::{
    access_control, delete_user, get_employees, get_user, internal_get_employees, login, post_user,
    put_user,
};

pub fn router(app_state: AppState) -> Router {
    let health_route = Router::new().route("/health", get(super::health::health));

    let user_routers = Router::new()
        .route(
            "/:id",
            delete(delete_user::delete_user)
                .put(put_user::update_user)
                .get(get_user::get_user),
        )
        .route("/", post(post_user::create_user))
        .route("/employees", get(get_employees::get_employees));

    let auth_routers = Router::new()
        .route("/login", post(login::login))
        .route("/access", get(access_control::access_control));

    let api_routes = Router::new()
        .nest("/users", user_routers)
        .nest("/auth", auth_routers)
        .route_layer(from_extractor_with_state::<Authorization, AppState>(
            app_state.clone(),
        ))
        .with_state(app_state.clone());

    let internal_api = Router::new()
        .route(
            "/internal/api/v1/employees",
            get(internal_get_employees::internal_get_employees),
        )
        .with_state(app_state);

    Router::new()
        .nest("/api/v1", api_routes)
        .merge(internal_api)
        .merge(health_route)
        .layer(TraceLayer::new_for_http())
}
