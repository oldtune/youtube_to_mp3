use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename(deserialize = "streamingData"))]
    streaming_data: StreamingData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    formats: Vec<Format>,
    #[serde(rename(deserialize = "adaptiveFormats"))]
    adaptive_formats: Vec<Format>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Format {
    itag: usize,
    url: String,
    #[serde(rename(deserialize = "mimeType"))]
    mime_type: String,
    bitrate: usize,
    #[serde(default)]
    width: Option<usize>,
    #[serde(default)]
    height: Option<usize>,
    #[serde(rename(deserialize = "lastModified"))]
    last_modified: String,
    #[serde(rename(deserialize = "contentLength"))]
    #[serde(default)]
    content_length: Option<String>,
    quality: String,
    #[serde(default)]
    fps: Option<usize>,
    #[serde(rename(deserialize = "qualityLabel"))]
    #[serde(default)]
    quality_label: Option<String>,
    #[serde(rename(deserialize = "projectionType"))]
    projection_type: String,
    #[serde(rename(deserialize = "averageBitrate"))]
    #[serde(default)]
    average_bitrate: Option<usize>,
    #[serde(rename(deserialize = "audioQuality"))]
    #[serde(default)]
    audio_quality: Option<String>,
    #[serde(rename(deserialize = "approxDurationMs"))]
    approx_duration_ms: String,
    #[serde(rename(deserialize = "audioSampleRate"))]
    #[serde(default)]
    audio_sample_rate: Option<String>,
    #[serde(rename(deserialize = "audioChannels"))]
    #[serde(default)]
    audio_channels: Option<usize>,
}
