use std::sync::Arc;

use secrecy::ExposeSecret;
use server::{
    PasswordHash, Username,
    features::user::{
        model::UserInsert,
        repository::{PgUserRepository, UserRepository},
    },
};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_add_user(db: PgPool) {
    // Arrange
    let expected_username = Username::new("test");
    let expected_password_hash = PasswordHash::new("password_hash");
    let expected_user_insert = UserInsert {
        username: expected_username,
        password_hash: expected_password_hash,
    };
    let repo = PgUserRepository::new(Arc::new(db));

    // Act
    let actual_result = repo.add_user(expected_user_insert).await;

    // Assert
    assert!(actual_result.is_ok());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_user_by_username_not_found(db: PgPool) {
    // Arrange
    let expected_username = Username::new("test");
    let repo = PgUserRepository::new(Arc::new(db));

    // Act
    let actual_result = repo.get_user_by_username(&expected_username).await;

    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_user_by_username_existing_username(db: PgPool) {
    // Arrange
    let expected_username = Username::new("test");
    let expected_password_hash = PasswordHash::new("password_hash");
    let expected_user_insert = UserInsert {
        username: expected_username.clone(),
        password_hash: expected_password_hash.clone(),
    };
    let repo = PgUserRepository::new(Arc::new(db));
    let _ = repo.add_user(expected_user_insert).await;

    // Act
    let actual_user = repo.get_user_by_username(&expected_username).await.unwrap();

    // Assert
    assert_eq!(actual_user.username, expected_username);
    assert_eq!(
        actual_user.password_hash.into_inner().expose_secret(),
        expected_password_hash.into_inner().expose_secret()
    );
}
