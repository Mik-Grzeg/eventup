use axum::Json;

use crate::middlewares::auth::Authorization;
use common_types::UserIdentifiers;

use super::errors::PublicError;

pub async fn access_control(
    Authorization(user_identifiers): Authorization,
) -> Result<Json<UserIdentifiers>, PublicError> {
    if let Some(identifiers) = user_identifiers {
        Ok(Json(identifiers))
    } else {
        Err(PublicError::Unauthorized)
    }
}
