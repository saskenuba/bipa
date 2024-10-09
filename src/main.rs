use log::LevelFilter;

pub mod application;
pub mod infrastructure;

#[tokio::main]
pub async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    infrastructure::server::serve().await
}
