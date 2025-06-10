use async_trait::async_trait;
use std::collections::HashSet;

use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use tokio::task;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    id: i64,
    pub username: String,
    grade: i64,
    realname: String,
    password: String,
    score: i64,
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct LeaderBoard {
    pub username: String,
    pub realname: Option<String>,
    pub grade: i64,
    pub score: i64,
    pub rank: i32,
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct BracketAndName {
    pub username: String,
    pub bracket: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("realname", &self.realname)
            .field("grade", &self.grade)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
                                 // hash--what this means
                                 // is when the user changes their password the
                                 // auth session becomes invalid.
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
    pub signup: Option<Signup>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Signup {
    pub grade: i64,
    pub realname: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: SqlitePool,
}

impl Backend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

impl Backend {
    pub async fn admin_insert_bracket(
        &self,
        bracket: crate::bracket::Bracket,
    ) -> Result<(), sqlx::Error> {
        let query = r#"
        INSERT OR REPLACE INTO admindata (bracket) VALUES ($1)
        "#;
        sqlx::query(query)
            .bind(serde_json::to_string(&bracket).unwrap())
            .execute(&self.db)
            .await?;
        Ok(())
    }
    pub async fn get_leaderboard(&self) -> Result<Vec<LeaderBoard>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT username, grade, realname, score, RANK() OVER (ORDER BY score DESC) AS rank
            FROM users
            "#,
        )
        .fetch_all(&self.db)
        .await?;

        let users = rows
            .iter()
            .map(|row| LeaderBoard {
                grade: row.get("grade"),
                username: row.get("username"),
                realname: row.get("realname"),
                score: row.get("score"),
                rank: row.get("rank"),
            })
            .collect();

        Ok(users)
    }

    pub async fn clear_scores(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET score = 0")
            .execute(&self.db)
            .await?;
        Ok(())
    }

    pub async fn gen_scores(&self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query("SELECT bracket FROM admindata")
            .fetch_all(&self.db)
            .await?;
        let bracket: String = rows[0].get("bracket");
        let offical_bracket: crate::bracket::Bracket = serde_json::from_str(&bracket).unwrap();

        let rows = sqlx::query("SELECT bracket, username FROM users")
            .fetch_all(&self.db)
            .await?;

        let users: Vec<BracketAndName> = rows
            .iter()
            .map(|row| BracketAndName {
                username: row.get("username"),
                bracket: row.get("bracket"),
            })
            .collect();

        for user in users {
            let string_bracket: String = user.bracket;
            let bracket: crate::bracket::Bracket = serde_json::from_str(&string_bracket).unwrap();
            let score = offical_bracket.calculate_score(bracket);
            let query = r#"
                INSERT INTO scores (username, score)
                VALUES ($1, $2)
            "#;

            sqlx::query(query)
                .bind(user.username)
                .bind(score)
                .execute(&self.db)
                .await?;
        }

        Ok(())
    }

    pub async fn add_bracket(
        &self,
        bracket: crate::bracket::Bracket,
        user: User,
    ) -> Result<(), sqlx::Error> {
        let bracket_string = serde_json::to_string(&bracket).unwrap();
        let query = r#"
                INSERT OR REPLACE INTO users (username, bracket)
                VALUES ($1, $2)
                "#;

        sqlx::query(query)
            .bind(user.username)
            .bind(bracket_string)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn get_bracket(&self, user: User) -> Result<crate::bracket::Bracket, sqlx::Error> {
        let row: Option<(String,)> =
            sqlx::query_as("select bracket from users where username = $1")
                .bind(&user.username)
                .fetch_optional(&self.db)
                .await?;

        let mut bracket: crate::bracket::Bracket =
            serde_json::from_str(&row.map(|(bracket,)| bracket).unwrap()).unwrap();

        Ok(bracket)
    }

    pub async fn add_user(
        &self,
        password: String,
        username: String,
        grade: i64,
        realname: String,
    ) -> Result<(), sqlx::Error> {
        tracing::info!("add_user");
        let exists: Option<User> = sqlx::query_as("select * from users where username = ? ")
            .bind(username.clone())
            .fetch_optional(&self.db)
            .await?;

        // If the username is already in use, don't do anythng
        if exists.is_some() {
            return Ok(());
        }

        let hash = generate_hash(&password);

        let query = r#"
        INSERT INTO users (username, grade, realname, password)
        VALUES ($1, $2, $3, $4)
        "#;

        // Execute the query
        sqlx::query(query)
            .bind(&username)
            .bind(grade)
            .bind(realname)
            .bind(hash)
            .execute(&self.db)
            .await?;

        let query = r#"
        INSERT INTO users_groups (user_id, group_id)
        values (
            (SELECT id FROM users WHERE username = $1),
            (SELECT id FROM groups WHERE name = 'users')
        )
        "#;

        sqlx::query(query).bind(&username).execute(&self.db).await?;

        let user: Option<User> = sqlx::query_as("select * from users where username = ? ")
            .bind(username)
            .fetch_optional(&self.db)
            .await?;

        println!("{:?}", user);

        Ok(())
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user: Option<Self::User> = sqlx::query_as("select * from users where username = ? ")
            .bind(creds.username)
            .fetch_optional(&self.db)
            .await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        task::spawn_blocking(|| {
            // We're using password-based authentication: this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as("select * from users where id = ?")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, FromRow)]
pub struct Permission {
    pub name: String,
}

impl From<&str> for Permission {
    fn from(name: &str) -> Self {
        Permission {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl AuthzBackend for Backend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let permissions: Vec<Self::Permission> = sqlx::query_as(
            r#"
            select distinct permissions.name
            from users
            join users_groups on users.id = users_groups.user_id
            join groups_permissions on users_groups.group_id = groups_permissions.group_id
            join permissions on groups_permissions.permission_id = permissions.id
            where users.id = ?
            "#,
        )
        .bind(user.id)
        .fetch_all(&self.db)
        .await?;

        Ok(permissions.into_iter().collect())
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<Backend>;
