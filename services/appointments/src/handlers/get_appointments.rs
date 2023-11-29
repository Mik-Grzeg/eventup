
use auth_extractor::AuthExtractor;
use super::errors::PublicError;

pub async fn get_appointments(
    AuthExtractor(user_identifiers): AuthExtractor,
) -> Result<(), PublicError> {
    tracing::info!("Request for user {user_identifiers:?}");

    Ok(())
}
