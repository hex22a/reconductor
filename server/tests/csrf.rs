use server::persistence::kv::{
    FredKvProvider,
    csrf::{CsrfRepository, CsrfStore},
};

async fn create_store() -> CsrfStore<FredKvProvider> {
    let kv = FredKvProvider::new(
        std::env::var("REDIS_URL").unwrap_or("redis://localhost:6379".to_string()),
        2,
    )
    .await
    .expect("failed to connect to Redis");
    CsrfStore::new(kv)
}

#[tokio::test]
async fn stores_csrf_under_correct_key() {
    // Arrange
    let store = create_store().await;
    let expected_token = "store_csrf_under_correct_key".to_string();

    // Act
    let actual_result = store.create_anonymous_csrf(expected_token).await.unwrap();

    //Assert
    assert_eq!(actual_result, ());
}

#[tokio::test]
async fn returns_false_for_missing_csrf() {
    // Arrange
    let store = create_store().await;
    let expected_token = "csrf_does_not_exist".to_string();

    // Act
    let actual_result = store.verify_anonymous_csrf(expected_token).await.unwrap();

    // Assert
    assert_eq!(actual_result, false);
}

#[tokio::test]
async fn gets_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "get_csrf_from_storage".to_string();
    store
        .create_anonymous_csrf(expected_token.clone())
        .await
        .unwrap();

    // Act
    let actual_user_session = store.verify_anonymous_csrf(expected_token).await.unwrap();

    // Assert
    assert_eq!(actual_user_session, true);
}

#[tokio::test]
async fn deletes_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "csrf_token_to_delete".to_string();
    store
        .create_anonymous_csrf(expected_token.clone())
        .await
        .unwrap();

    // Act
    store
        .delete_anonymous_csrf(expected_token.clone())
        .await
        .unwrap();

    // Assert
    let result = store.verify_anonymous_csrf(expected_token).await.unwrap();
    assert_eq!(result, false);
}
