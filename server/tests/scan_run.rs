use std::sync::Arc;

use server::features::scan_run::repository::{PgScanRunRepository, ScanRunRepository};
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_scan_run(db: &PgPool) -> (Uuid, Uuid, Uuid) {
    let expected_user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.users
            (username, password_hash)
        VALUES ('test_user', 'password_hash')
        RETURNING id
        "#,
    )
    .fetch_one(db)
    .await
    .unwrap();
    let expected_project_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.projects
            (name, owner_id)
        VALUES ('test_project', $1)
        RETURNING id
        "#,
    )
    .bind(expected_user_id)
    .fetch_one(db)
    .await
    .unwrap();
    let expected_scan_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scans
            (project_id, target, schedule)
        VALUES ($1, '192.168.0.1', '* * * * *')
        RETURNING id
        "#,
    )
    .bind(expected_project_id)
    .fetch_one(db)
    .await
    .unwrap();
    let expected_scan_run_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scan_runs
        (scan_id)
        VALUES ($1)
        RETURNING id
        "#,
    )
    .bind(expected_scan_id)
    .fetch_one(db)
    .await
    .unwrap();
    (expected_scan_run_id, expected_scan_id, expected_project_id)
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id(db: PgPool) {
    // Arrange
    let (expected_scan_run_id, _expected_scan_id, _expected_project_id) = setup_scan_run(&db).await;
    let repo = PgScanRunRepository::new(Arc::new(db));
    // Act
    let actual_scan_run = repo.get_scan_run(&expected_scan_run_id).await.unwrap();
    // Assert
    assert_eq!(actual_scan_run.id, expected_scan_run_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id_not_found(db: PgPool) {
    // Arrange
    let expected_scan_run_id = Uuid::now_v7();
    let repo = PgScanRunRepository::new(Arc::new(db));
    // Act
    let actual_result = repo.get_scan_run(&expected_scan_run_id).await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_list_scans(db: PgPool) {
    // Arrange
    let expected_limit: i64 = 1;
    let (_expected_scan_run_id, expected_scan_id, _expected_project_id) = setup_scan_run(&db).await;
    let repo = PgScanRunRepository::new(Arc::new(db));
    // Act
    let actual_projects = repo
        .list_scan_runs(&expected_scan_id, None, expected_limit)
        .await
        .unwrap();
    // Assert
    assert_eq!(actual_projects.len(), expected_limit as usize);
}
