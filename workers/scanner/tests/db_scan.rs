use scanner::db::scan::{PgScanRepository, ScanRepository, ScanStatus};
use sqlx::{PgPool, types::ipnetwork::IpNetwork};
use uuid::Uuid;

async fn setup_scans(db: &PgPool) -> Uuid {
    let expected_target: IpNetwork = "192.168.50.1/16".parse().unwrap();
    let expected_user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.users
            (username, password_hash)
        VALUES ('test_user', 'password_hash')
        RETURNING id
        "#
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
        "#
    )
    .bind(expected_user_id)
    .fetch_one(db)
    .await
    .unwrap();
    let expected_scan_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scans
            (project_id, target, next_run_at, schedule)
        VALUES ($1, $2, NOW() - INTERVAL '1 minute', '5 * * * *')
        RETURNING id
        "#
    )
    .bind(expected_project_id)
    .bind(expected_target)
    .fetch_one(db)
    .await
    .unwrap();
    return expected_scan_id;
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_update_scan_status_done(db: PgPool) {
    // Arrange
    let expected_status = ScanStatus::Done;
    let repo = PgScanRepository { db: db.clone() };
    let expected_scan_id: Uuid = setup_scans(&db).await;
    // Act
    let actual_result = repo.update_scan_status(expected_scan_id, expected_status).await;
    // Assert
    assert_eq!(actual_result.unwrap(), ());
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_update_scan_status_in_progress(db: PgPool) {
    // Arrange
    let expected_status = ScanStatus::InProgress;
    let repo = PgScanRepository { db: db.clone() };
    let expected_scan_id: Uuid = setup_scans(&db).await;
    // Act
    let actual_result = repo.update_scan_status(expected_scan_id, expected_status).await;
    // Assert
    assert_eq!(actual_result.unwrap(), ());
}
