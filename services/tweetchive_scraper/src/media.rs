use crate::AppState;
use bytes::Bytes;
use color_eyre::{Report, Result};
use memmap2::{Mmap, MmapOptions};
use reqwest::get;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::{error, instrument};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[instrument]
pub async fn download_video_media(id: u64, ua: String, display_url: String) -> Result<Mmap> {
    if let YoutubeDlOutput::SingleVideo(sv) = YoutubeDl::new(display_url)
        .user_agent(ua)
        .extra_arg("-o [%(id)s].%(ext)s")
        .extra_arg("--embed-metadata")
        .extra_arg("--embed-thumbnail")
        .extra_arg("--embed-subs")
        .extra_arg("--remux-video \"mkv\"")
        .run_async()
        .await?
    {
        let map = match unsafe { MmapOptions::new().populate().map(format!("{id}.mkv")) } {
            Ok(m) => m,
            Err(why) => {
                error!(media = id, error = why, "Failed to get thing.");
                return Err(Report::new(why));
            }
        };

        Ok(map)
    }

    error!(media = id, "This is a playlist!");
    Err(Report::msg("Playlist Not Supported."))
}

#[derive(Clone, Debug, PartialEq)]
pub struct DownloadedImage {
    pub id: u64,
    pub extension: String,
    pub data: Bytes,
}

#[instrument]
pub async fn download_image_media(id: u64, source: String) -> Result<DownloadedImage> {
    let download = get(source).await?;
    let extension = download
        .headers()
        .get("content-type")
        .map(|x| x.to_str().map(|x| x.split("/").nth(1)).ok())
        .flatten()
        .flatten()
        .unwrap_or("jpeg")
        .to_string();
    let data = download.bytes().await?;
    Ok(DownloadedImage {
        id,
        extension,
        data,
    })
}

#[instrument]
pub async fn upload(
    state: Arc<AppState>,
    id: u64,
    extension: String,
    data: impl AsRef<[u8]>,
) -> Result<()> {
    let mut hasher = Sha256::new();
    hasher.update(data.as_ref());
    let result = hasher.finalize();
    let object_path = format!("/{id}.{}", &extension);
    let mut tags = Vec::new();

    tags.push(
        ()
    )
    state.s3.put_object(&object_path, data.as_ref()).await?;
    state.s3.put_object_tagging(&object_path, )
}
