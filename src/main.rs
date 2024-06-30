use std::error::Error;

use futures::SinkExt;
use simple_redis::stream_handler;
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::level_filters::LevelFilter;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_log_context()?;

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            match stream_handler::handle_stream(stream).await {
                Ok(_) => {
                    info!("Connection from {} exited", raddr);
                }
                Err(e) => {
                    warn!("handle error for {}: {:?}", raddr, e);
                }
            }
        });
    }
}

fn init_log_context() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into());
    let subscriber = Registry::default()
        .with(
            fmt::layer()
                .compact()
                .with_ansi(true)
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(false)
                .with_target(true),
        )
        .with(filter);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}
