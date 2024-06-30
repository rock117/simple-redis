use crate::command;
use crate::context::AppContext;
use crate::error::RedisError;
use crate::resp::nulls;
use crate::resp::Resp::Nulls;
use crate::resp::{Resp, RespCodec};
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{info, warn};

pub async fn handle_stream(mut stream: TcpStream) -> Result<(), RedisError> {
    let mut framed = Framed::new(stream, RespCodec);
    loop {
        match framed.next().await {
            Some(Ok(Resp::BulkStrings(bulk_strings))) => {
                info!("Received frame: {}", bulk_strings);
                let cmd = command::parse_command(bulk_strings);
                if let Some(cmd) = cmd {
                    let resp = cmd.execute(&AppContext)?;
                    framed.send(resp).await?;
                }
            }
            Some(Ok(frame)) => {
                warn!("invalid command args: {:?}", frame);
                framed.send(frame).await?;
            }
            Some(Err(e)) => {
                if is_fatal_error(&e) {
                    return Err(e);
                }
                warn!("Failed to handle frame: {:?}", e);
                framed.send(Nulls(nulls::Nulls)).await?; // TODO
            }
            None => return Ok(()), // no data available to read
        }
    }
}

fn is_fatal_error(error: &RedisError) -> bool {
    match error {
        _ => false,
        RedisError::IOError(_) => true,
    }
}
