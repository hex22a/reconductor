use server::persistence::kv::{
    FredKvProvider,
    session::{SessionError, SessionRepository, SessionStore, UserSession},
};
use uuid::Uuid;

async fn create_store() -> SessionStore<FredKvProvider> {
    let kv = FredKvProvider::new(
        std::env::var("REDIS_URL").unwrap_or("redis://localhost:6379".to_string()),
        2,
    )
    .await
    .expect("failed to connect to Redis");
    SessionStore::new(kv)
}

fn create_user_session_fixture(token: String) -> UserSession {
    UserSession {
        token: token,
        user_id: Uuid::now_v7(),
        username: "testuser".to_string(),
        csrf_token: "csrf_token".to_string(),
    }
}

#[tokio::test]
async fn stores_session_under_correct_key() {
    // Arrange
    let store = create_store().await;
    let expected_token = "store_under_correct_key".to_string();
    let expected_user_session = create_user_session_fixture(expected_token);

    // Act
    let actual_result = store
        .create_user_session(expected_user_session)
        .await
        .unwrap();

    //Assert
    assert_eq!(actual_result, ());
}

#[tokio::test]
async fn returns_not_found_error_for_missing_session() {
    // Arrange
    let store = create_store().await;
    let expected_token = "does_not_exist".to_string();

    // Act
    let actual_result = store.get_user_session(expected_token).await;

    // Assert
    assert!(matches!(actual_result, Err(SessionError::NotFound)));
}

#[tokio::test]
async fn gets_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "get_session_from_storage".to_string();
    let expected_user_session = create_user_session_fixture(expected_token.clone());
    store
        .create_user_session(expected_user_session.clone())
        .await
        .unwrap();

    // Act
    let actual_user_session = store.get_user_session(expected_token).await.unwrap();

    // Assert
    assert_eq!(actual_user_session, expected_user_session);
}

#[tokio::test]
async fn deletes_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "token_to_delete".to_string();
    let expected_user_session = create_user_session_fixture(expected_token.clone());
    store
        .create_user_session(expected_user_session)
        .await
        .unwrap();

    // Act
    store
        .delete_user_session(expected_token.clone())
        .await
        .unwrap();

    // Assert
    let result = store.get_user_session(expected_token).await;
    assert!(matches!(result, Err(SessionError::NotFound)));
}
