use actix_identity::Identity;
use serde::{Serialize};
use sqlx::SqlitePool;

use crate::{
    auth::{
        repositories::Group,
        repositories::{SqliteUserRepository, UserRepository},
    },
    
};

pub struct Credentials<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Serialize)]
pub struct UserInfo {
    displayname: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait AuthService {
    async fn register(
        &self,
        username: &str,
        password: &str,
        displayname: &str,
        group: Group,
        avatar_seed: &str,
    ) -> bool;

    async fn verify_login<'a>(&self, credentials: &Credentials<'a>) -> bool;
    async fn inject_user_info(&self, ctx: &mut tera::Context, username: Option<Identity>);
    async fn has_admin_account(&self) ->bool;
}

pub struct AuthServiceImpl<U: UserRepository> {
    user_repository: U,
}

impl AuthServiceImpl<SqliteUserRepository> {
    pub fn new(db: SqlitePool) -> Self {
        let user_repository = SqliteUserRepository::new(db);
        Self { user_repository }
    }
}

impl<U> AuthService for AuthServiceImpl<U>
where
    U: UserRepository,
{
    async fn register(
        &self,
        username: &str,
        password: &str,
        displayname: &str,
        group: Group,
        avatar_seed: &str,
    ) -> bool {
        let hashed_password = password_auth::generate_hash(password);
        let result = self
            .user_repository
            .create(
                username,
                hashed_password.as_str(),
                displayname,
                group,
                avatar_seed,
            )
            .await;
        result.is_some()
    }

    async fn verify_login(&self, credentials: &Credentials<'_>) -> bool {
        match self
            .user_repository
            .get_user_by_username(credentials.username)
            .await
        {
            Some(user) => {
                password_auth::verify_password(credentials.password, user.password.as_str()).is_ok()
            }
            None => false,
        }
    }

    async fn inject_user_info(&self, ctx: &mut tera::Context, id: Option<Identity>) {
        let userinfo = match id.map(|i| i.id()) {
            Some(Ok(username)) => self
                .user_repository
                .get_user_by_username(username.as_str())
                .await
                .map(|u| UserInfo {
                    displayname: u.displayname,
                }),
            _ => None,
        };
        if let Some(userinfo) = userinfo {
            ctx.insert("user", &userinfo);
        }
    }

    async fn has_admin_account(&self) -> bool {
        false
    }
}

pub type DefaultAuthService = AuthServiceImpl<SqliteUserRepository>;
