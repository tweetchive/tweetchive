use crate::AppState;
use bytes::Bytes;
use color_eyre::{Report, Result};
use memmap2::{Mmap, MmapOptions};
use reqwest::{get, Client};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tracing::{error, instrument};
use twtscrape::scrape::Scraper;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

pub struct DownloadedAnimatedContent {
    pub data: Mmap,
    pub content_type: String,
}

#[instrument]
pub async fn download_video_media(
    ua: impl AsRef<str>,
    display_url: impl AsRef<str>,
) -> Result<DownloadedAnimatedContent> {
    let filename_hash = seahash::hash(display_url.as_ref().as_ref());

    if let YoutubeDlOutput::SingleVideo(_) = YoutubeDl::new(display_url.as_ref())
        .user_agent(ua)
        .extra_arg(format!("-o {filename_hash}.%(ext)s"))
        .extra_arg("--embed-metadata")
        .extra_arg("--embed-thumbnail")
        .extra_arg("--embed-subs")
        .extra_arg("--remux-video \"mkv\"")
        .run_async()
        .await?
    {
        let map = match unsafe {
            MmapOptions::new()
                .populate()
                .map(format!("{filename_hash}.mkv"))
        } {
            Ok(m) => m,
            Err(why) => {
                error!(media = id, error = why, "Failed to get thing.");
                return Err(Report::new(why));
            }
        };

        Ok(DownloadedAnimatedContent {
            data: map,
            content_type: "video/x-matroska".to_string(),
        })
    }

    error!(media = id, "This is a playlist!");
    Err(Report::msg("Playlist Not Supported."))
}

#[derive(Clone, Debug, PartialEq)]
pub struct DownloadedImage {
    pub content_type: String,
    pub data: Bytes,
}

#[instrument]
pub async fn download_image_media(
    scraper: &Scraper,
    source: impl AsRef<str>,
) -> Result<DownloadedImage> {
    let download = scraper
        .api_req_raw_request(scraper.make_get_req(source))
        .await?;
    let content_type = download
        .headers()
        .get("content-type")
        .map(|x| x.to_str().ok())
        .flatten()
        .unwrap_or("image/jpeg")
        .to_string();
    let data = download.bytes().await?;
    Ok(DownloadedImage { content_type, data })
}

#[instrument]
pub async fn upload(
    state: Arc<AppState>,
    id: impl AsRef<str>,
    mime_type: impl AsRef<str>,
    data: impl AsRef<[u8]>,
) -> Result<()> {
    let id = id.as_ref();
    let mime_type = mime_type.as_ref();
    let response = state
        .s3
        .put_object_with_content_type(format!("/{}", id), data.as_ref(), mime_type)
        .await?;
    if response.status_code() == 200 {
        return Ok(());
    } else {
        error!(
            file = id,
            error = response.status_code(),
            "Error uploading file"
        );
        Err(Report::msg(format!(
            "Uploading file {}: {}",
            id,
            response.status_code()
        )))
    }
}
