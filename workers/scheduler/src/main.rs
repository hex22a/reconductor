mod config;
mod db;

fn main() {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
}
