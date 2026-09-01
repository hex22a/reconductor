use std::str::FromStr;

use scanner::db::scan::{
    PgScanRepository, ScanHostInsert, ScanPortInsert, ScanRepository, ScanStatus,
};
use sqlx::{
    PgPool,
    types::{ipnetwork::IpNetwork, mac_address::MacAddress},
};
use uuid::Uuid;

async fn setup_scans(db: &PgPool) -> Uuid {
    let expected_target: IpNetwork = "192.168.50.1/16".parse().unwrap();
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
            (project_id, target, next_run_at, schedule)
        VALUES ($1, $2, NOW() - INTERVAL '1 minute', '5 * * * *')
        RETURNING id
        "#,
    )
    .bind(expected_project_id)
    .bind(expected_target)
    .fetch_one(db)
    .await
    .unwrap();
    expected_scan_id
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_update_scan_status_done(db: PgPool) {
    // Arrange
    let expected_status = ScanStatus::Done;
    let repo = PgScanRepository { db: db.clone() };
    let expected_scan_id: Uuid = setup_scans(&db).await;
    // Act
    let actual_result = repo
        .update_scan_status(expected_scan_id, expected_status)
        .await;
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
    let actual_result = repo
        .update_scan_status(expected_scan_id, expected_status)
        .await;
    // Assert
    assert_eq!(actual_result.unwrap(), ());
}

#[sqlx::test(migrations = "../../migrations/")]
async fn test_store_scan_results(db: PgPool) {
    // Arrange
    let expected_host_ip: IpNetwork = "192.168.0.1".parse().unwrap();
    let expected_host_mac_address: MacAddress = MacAddress::from_str("08:00:2b:01:02:03").unwrap();
    let expected_ports: Vec<ScanPortInsert> = vec![
        ScanPortInsert {
            port: 22,
            protocol: Some("ssh".to_string()),
            state: Some("up".to_string()),
            service: Some("ssh".to_string()),
            product: Some("ssh".to_string()),
            version: Some("1.0.1".to_string()),
        },
        ScanPortInsert {
            port: 80,
            protocol: Some("http".to_string()),
            state: Some("up".to_string()),
            service: Some("server".to_string()),
            product: Some("bun".to_string()),
            version: Some("1.0.1".to_string()),
        },
    ];
    let expected_hosts: Vec<ScanHostInsert> = vec![ScanHostInsert {
        ip: Some(expected_host_ip),
        mac: Some(expected_host_mac_address),
        vendor: Some("linux".to_string()),
        hostname: Some("durandal".to_string()),
        os_match: Some("debian".to_string()),
        os_accuracy: Some(90),
        ports: expected_ports,
    }];
    let repo = PgScanRepository { db: db.clone() };
    let expected_scan_id: Uuid = setup_scans(&db).await;
    // Act
    let actual_result = repo
        .store_scan_results(expected_scan_id, expected_hosts)
        .await;
    // Assert
    assert_eq!(actual_result.unwrap(), ());
}
