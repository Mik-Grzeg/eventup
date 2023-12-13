use common_types::{UserIdentifiers, UserRoles};
use tracing_subscriber::EnvFilter;
use uuid::uuid;

pub fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::from_default_env();

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub fn regular_user_identifiers() -> UserIdentifiers {
    UserIdentifiers {
        id: uuid!("b9ee058b-3143-4176-851b-a60cde9d06eb"),
        email: "user@mail.com".into(),
        role: UserRoles::Regular,
    }
}

pub fn admin_user_identifiers() -> UserIdentifiers {
    UserIdentifiers {
        id: uuid!("b9ee058b-3143-4176-851b-a60cde9d06ed"),
        email: "admin@mail.com".into(),
        role: UserRoles::Admin,
    }
}
