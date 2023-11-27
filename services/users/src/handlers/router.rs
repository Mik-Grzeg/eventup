use crate::app_state::AppState;
use axum::{
    routing::{delete, get, post},
    Router,
};

use super::{delete_user, get_user, post_user, put_user};

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
        .with_state(app_state);

    let api_routes = Router::new().nest("/users", user_routers);

    Router::new()
        .nest("/api/v1", api_routes)
        .merge(health_route)
}
