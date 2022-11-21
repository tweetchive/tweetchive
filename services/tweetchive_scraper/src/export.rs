use crate::AppState;
use async_compression::tokio::write::ZstdEncoder;
use std::sync::Arc;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufReader};
use tracing::{error, instrument, warn};

#[instrument]
pub async fn upload_done_user(state: Arc<AppState>) {
    loop {
        match state.user_done_channel.receiver.recv_async().await {
            Ok(user) => {
                let mut data = match rkyv::to_bytes(&user) {
                    Ok(d) => d,
                    Err(why) => {
                        error!(error = why, id = user.archival_id, "Error serialize user");
                        continue;
                    }
                };
                let mut enc = ZstdEncoder::new(Vec::new());
                if let Err(why) = enc.write_all(data.as_ref()).await {
                    error!(error = why, id = user.archival_id, "Error compress user");
                    continue;
                }
                if let Err(why) = enc.shutdown().await {
                    error!(error = why, id = user.archival_id, "Error compress user");
                    continue;
                }
                let data = enc.into_inner();

                // TODO: Send to RabbitMQ
                let
            }
            Err(why) => {
                warn!(error = why, "Receiving User Dones");
                continue;
            }
        }
    }
}
