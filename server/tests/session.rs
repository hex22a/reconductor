use std::sync::Arc;

use server::{
    Config,
    features::session::{
        model::UserSession,
        repository::{SessionRepository, SessionRepositoryError, SessionStore},
    },
    infra::persistence::kv::{self, FredKvProvider, KvConfig},
};
use uuid::Uuid;

async fn create_store() -> SessionStore<FredKvProvider> {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("Unable to read environment variables");
    let kv = Arc::new(
        kv::init_kv(KvConfig {
            username: config.kv_username,
            password: config.kv_password,
            host: config.kv_host,
            port: config.kv_port,
            database: config.kv_db,
        })
        .await,
    );
    SessionStore::new(kv)
}

fn create_user_session_fixture(token: String) -> UserSession {
    UserSession {
        token,
        user_id: Uuid::now_v7(),
        username: "testuser".to_string(),
        csrf_token: "csrf_token".to_string(),
        csrf_cookie: "csrf_cookei".to_string(),
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
    let expected_token = "does_not_exist";

    // Act
    let actual_result = store.get_user_session(expected_token).await;

    // Assert
    assert!(matches!(
        actual_result,
        Err(SessionRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn gets_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "get_session_from_storage";
    let expected_user_session = create_user_session_fixture(expected_token.to_string());
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
    let expected_token = "token_to_delete";
    let expected_user_session = create_user_session_fixture(expected_token.to_string());
    store
        .create_user_session(expected_user_session)
        .await
        .unwrap();

    // Act
    store.delete_user_session(expected_token).await.unwrap();

    // Assert
    let result = store.get_user_session(expected_token).await;
    assert!(matches!(result, Err(SessionRepositoryError::NotFound)));
}
