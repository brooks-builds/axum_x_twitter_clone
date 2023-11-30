use std::net::IpAddr;

use eyre::Result;
use router::create_main_router;

mod router;

pub struct App {
    address: IpAddr,
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = IpAddr::from([127, 0, 0, 1]);

        tracing_subscriber::fmt::init();

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        let router = create_main_router();
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;

        tracing::info!("Server listening on port {}", self.port);

        axum::serve(listener, router).await?;

        Ok(())
    }
}
