mod config;
mod db;
mod scheduler;
mod queue;

fn main() {
    dotenvy::dotenv().ok();
    println!("Hello, world!");
}
