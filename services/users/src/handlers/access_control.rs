use axum::Json;

use crate::middlewares::auth::RequireAuth;
use common_types::UserIdentifiers;

pub async fn access_control(RequireAuth(user_identifiers): RequireAuth) -> Json<UserIdentifiers> {
    Json(user_identifiers)
}
