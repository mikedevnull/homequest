use sqlx::SqlitePool;

#[derive(Debug, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
pub enum Group {
    Admin,
    User,
}

#[derive(Debug, PartialEq, Eq, sqlx::Type)]
pub struct User {
    id: i64,
    pub username: String,
    pub displayname: String,
    pub password: String,
    pub group: Group,
    pub avatar_seed: String,
}

#[cfg_attr(test, mockall::automock)]
pub trait UserRepository {
    async fn create(
        &self,
        username: &str,
        password: &str,
        displayname: &str,
        group: Group,
        avatar_seed: &str,
    ) -> Option<i64>;

    async fn get_user_by_username(&self, username: &str) -> Option<User>;
}

pub struct SqliteUserRepository {
    db: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(db: SqlitePool) -> Self {
        SqliteUserRepository { db }
    }
}

impl UserRepository for SqliteUserRepository {
    async fn create(
        &self,
        username: &str,
        password: &str,
        displayname: &str,
        group: Group,
        avatar_seed: &str,
    ) -> Option<i64> {
        let result = sqlx::query!(
        "INSERT INTO users (username, password, displayname, \"group\", avatar_seed) VALUES($1,$2,$3,$4,$5)",
        &username,
        &password,
        &displayname,
        &group,
        &avatar_seed
    ).execute(&self.db).await;

        match result {
            Ok(query_result) => Some(query_result.last_insert_rowid()),
            Err(_) => None,
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT id, username, password, displayname, \"group\" as \"group: Group\", avatar_seed FROM users WHERE username == $1", username).fetch_one(&self.db).await.ok()
    }
}

#[cfg(test)]
mod test {
    use crate::auth::repositories::{Group, SqliteUserRepository, User, UserRepository};

    #[sqlx::test]
    async fn create_user_in_database_succeeds(pool: sqlx::Pool<sqlx::Sqlite>) {
        let repo = SqliteUserRepository::new(pool.clone());
        let result = repo
            .create("foo", "bar", "FooBar", Group::Admin, "123456")
            .await;
        assert!(result.is_some());
        let user = sqlx::query_as!(User, "SELECT id, username, password, displayname, \"group\" as \"group: Group\", avatar_seed FROM users WHERE id == $1", result.unwrap()).fetch_one(&pool).await.unwrap();
        assert_eq!(user.username, "foo");
        assert_eq!(user.password, "bar");
        assert_eq!(user.displayname, "FooBar");
        assert_eq!(user.group, Group::Admin);
        assert_eq!(user.avatar_seed, "123456");
    }

    #[sqlx::test]
    async fn create_user_fails_for_existing_username(pool: sqlx::Pool<sqlx::Sqlite>) {
        let repo = SqliteUserRepository::new(pool.clone());
        let pre_existing_user_id = repo
            .create("foo", "bar", "FooBar", Group::Admin, "123456")
            .await
            .unwrap();
        let failed_user = repo
            .create("foo", "baz", "blubb", Group::User, "none")
            .await;
        assert!(failed_user.is_none());

        let pre_existing_user = sqlx::query_as!(User, "SELECT id, username, password, displayname, \"group\" as \"group: Group\", avatar_seed FROM users WHERE id == $1", &pre_existing_user_id).fetch_one(&pool).await.unwrap();
        assert_eq!(pre_existing_user.username, "foo");
        assert_eq!(pre_existing_user.password, "bar");
        assert_eq!(pre_existing_user.displayname, "FooBar");
        assert_eq!(pre_existing_user.group, Group::Admin);
        assert_eq!(pre_existing_user.avatar_seed, "123456");
    }
}
