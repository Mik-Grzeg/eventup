use crate::app_state::AppState;
use auth_extractor::AuthorizationControl;
use axum::{
    middleware::from_extractor_with_state,
    routing::{get, post, put},
    Router,
};

use super::{
    appointments::{
        delete_appointment, get_appointments, get_free_slots_for_day, post_appointment,
    },
    schedules::{get_schedules, post_schedule},
    services::{get_service, get_services, post_service, put_service},
};
use tower_http::trace::TraceLayer;

pub fn router(app_state: AppState) -> Router {
    let health_route = Router::new().route("/health", get(super::health::health));

    let services_router = Router::new()
        .route(
            "/",
            post(post_service::post_service).get(get_services::get_services),
        )
        .route(
            "/:id",
            put(put_service::put_service).get(get_service::get_service),
        )
        .route(
            "/:id/appointments/free",
            get(get_free_slots_for_day::get_free_slots_for_day),
        );

    let appointments_router = Router::new()
        .route("/", post(post_appointment::create_appointment))
        .route("/free", get(get_free_slots_for_day::get_free_slots_for_day))
        .route(
            "/:id",
            get(get_appointments::get_appointments_for_user)
                .delete(delete_appointment::delete_appointment),
        );

    let schedules_router = Router::new().route(
        "/",
        post(post_schedule::post_schedule).get(get_schedules::get_schedules),
    );

    let api_routers = Router::new()
        .nest("/appointments", appointments_router)
        .nest("/services", services_router)
        .nest("/employee_schedules", schedules_router)
        .route_layer(from_extractor_with_state::<AuthorizationControl, AppState>(
            app_state.clone(),
        ))
        .with_state(app_state.clone());

    Router::new()
        .nest("/api/v1", api_routers)
        .merge(health_route)
        .layer(TraceLayer::new_for_http())
}
