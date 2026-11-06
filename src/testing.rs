use sqlx::SqlitePool;

use crate::auth::AuthService;
use crate::auth::DefaultAuthService;
use crate::auth::Group;

pub struct TestCredentials {
    pub username: String,
    pub password: String,
    pub displayname: String,
}

pub async fn create_test_admin_account<'a>(pool: SqlitePool) -> TestCredentials {
    let credentials = TestCredentials {
        username: "foo".to_string(),
        password: "bar".to_string(),
        displayname: "baz".to_string(),
    };
    let auth_service = DefaultAuthService::new(pool);
    auth_service
        .register(
            &credentials.username,
            &credentials.password,
            &credentials.displayname,
            Group::Admin,
            &credentials.username,
        )
        .await;
    credentials
}
