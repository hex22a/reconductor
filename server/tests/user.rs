use server::infra::persistence::db::user::{PgUserRepository, UserInsert, UserRepository};
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_add_user(db: PgPool) {
    // Arrange
    let expected_username: String = "test".to_string();
    let expected_password_hash: String = "password_hash".to_string();
    let expected_user_insert = UserInsert {
        username: expected_username,
        password_hash: expected_password_hash,
    };
    let repo = PgUserRepository { db: db.clone() };
    // Act
    let actual_result = repo.add_user(expected_user_insert).await.unwrap();
    // Assert
    assert_eq!(actual_result, ());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_user_by_username_not_found(db: PgPool) {
    // Arrange
    let expected_username: String = "test".to_string();
    let repo = PgUserRepository { db: db.clone() };
    // Act
    let actual_result = repo.get_user_by_username(&expected_username).await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_user_by_username_existing_username(db: PgPool) {
    // Arrange
    let expected_username: String = "test".to_string();
    let expected_password_hash: String = "password_hash".to_string();
    let expected_user_insert = UserInsert {
        username: expected_username.clone(),
        password_hash: expected_password_hash.clone(),
    };
    let repo = PgUserRepository { db: db.clone() };
    let _ = repo.add_user(expected_user_insert).await;
    // Act
    let actual_user = repo.get_user_by_username(&expected_username).await.unwrap();
    // Assert
    assert_eq!(actual_user.username, expected_username);
    assert_eq!(actual_user.password_hash, expected_password_hash);
}
