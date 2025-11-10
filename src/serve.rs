use crate::config::load_config;
use axum::Router;
use tower_http::services::ServeDir;

#[tokio::main]
pub async fn run(dev_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    if dev_mode {
        println!("starting donut planet server in dev mode...");
        println!(
            "{}:{}{}",
            config.server.domain, config.server.port, config.server.root_path
        );
    }

    let app = Router::new()
        .nest_service(
            &config.server.root_path,
            ServeDir::new(&config.directories.output),
        )
        .nest_service(
            &config.server.images_path,
            ServeDir::new(&config.directories.processed_images),
        );

    let addr = format!("{}:{}", &config.server.domain, &config.server.port);

    println!(
        "Server running on http://{}{}",
        addr, &config.server.root_path
    );
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
