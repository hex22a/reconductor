pub mod db;

#[ctor::ctor]
fn init() {
    dotenvy::dotenv().ok();
}
