use super::errors::PublicError;
use auth_extractor::AuthExtractor;

pub async fn get_appointments(
    AuthExtractor(user_identifiers): AuthExtractor,
) -> Result<(), PublicError> {
    tracing::info!("Request for user {user_identifiers:?}");

    Ok(())
}
