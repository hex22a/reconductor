use scheduler::features::scan::repository::{PgScanRepository, ScanRepository};
use sqlx::{PgPool, types::ipnetwork::IpNetwork};
use uuid::Uuid;

async fn setup_project(db: &PgPool) -> Uuid {
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
    expected_project_id
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_fetch_due_scans_return_due_scans(db: PgPool) {
    // Arrange
    let repo = PgScanRepository { db: db.clone() };
    let expected_target: IpNetwork = "192.168.50.1/16".parse().unwrap();
    let expected_project_id: Uuid = setup_project(&db).await;
    sqlx::query(
        r#"
        INSERT INTO recon.scans
            (project_id, target, next_run_at, schedule)
        VALUES ($1, $2, NOW() - INTERVAL '1 minute', '5 * * * *')
        "#,
    )
    .bind(expected_project_id)
    .bind(expected_target)
    .execute(&db)
    .await
    .unwrap();
    // Act
    let scans = repo.fetch_due_scans().await.unwrap();
    // Assert
    assert_eq!(scans.len(), 1);
    assert_eq!(scans[0].target, expected_target)
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_fetch_due_scans_no_schedule(db: PgPool) {
    // Arrange
    let repo = PgScanRepository { db: db.clone() };
    let expected_target: IpNetwork = "192.168.50.1/16".parse().unwrap();
    let expected_project_id: Uuid = setup_project(&db).await;
    sqlx::query(
        r#"
        INSERT INTO recon.scans
            (project_id, target, next_run_at)
        VALUES ($1, $2, NOW() - INTERVAL '1 minute')
        "#,
    )
    .bind(expected_project_id)
    .bind(expected_target)
    .execute(&db)
    .await
    .unwrap();
    // Act
    let scans = repo.fetch_due_scans().await.unwrap();
    // Assert
    assert_eq!(scans.len(), 0);
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_fetch_due_scans_next_run_in_future(db: PgPool) {
    // Arrange
    let repo = PgScanRepository { db: db.clone() };
    let expected_target: IpNetwork = "192.168.50.1/16".parse().unwrap();
    let expected_project_id: Uuid = setup_project(&db).await;
    sqlx::query(
        r#"
        INSERT INTO recon.scans
            (project_id, target, next_run_at, schedule)
        VALUES ($1, $2, NOW() + INTERVAL '1 minute', '5 * * * *')
        "#,
    )
    .bind(expected_project_id)
    .bind(expected_target)
    .execute(&db)
    .await
    .unwrap();
    // Act
    let scans = repo.fetch_due_scans().await.unwrap();
    // Assert
    assert_eq!(scans.len(), 0);
}
