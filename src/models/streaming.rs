use std::collections::HashMap;

use crate::de::*;

/// Represents transcode information for a RealDebrid file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Transcode {
    /// M3U8 Live Streaming format
    pub apple: HashMap<String, String>,
    /// MPD Live Streaming format
    pub dash: HashMap<String, String>,
    #[serde(rename = "liveMP4")]
    /// Live MP4
    pub live_mp4: HashMap<String, String>,
    /// Live H264 WebM
    #[serde(rename = "h264WebM")]
    pub h264_webm: HashMap<String, String>,
}

/// Represents media info for a RealDebrid file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MediaInfo {
    /// Cleaned filename
    pub filename: String,
    /// Host the file is hosted on
    pub hoster: String,
    /// Original content link
    pub link: String,
    /// Type of file
    #[serde(rename = "type")]
    pub type_: MediaType,
    /// Season of the file, if applicable
    pub season: Option<String>,
    /// Episode of the file, if applicable
    pub episode: Option<String>,
    /// Year of the file, if applicable
    pub year: Option<String>,
    /// Media duration in seconds
    pub duration: f64,
    /// Bitrate of the media file
    pub bitrate: u64,
    /// Original filesize in bytes
    pub size: u64,
    /// Track information
    pub details: MediaDetails,
    /// URL of the backdrop image, if applicable
    pub backdrop_path: Option<String>,
    /// URL of the poster image, if applicable
    pub poster_path: Option<String>,
    /// URL of the music image in HD, if applicable
    pub audio_image: Option<String>,
    #[serde(rename = "baseUrl")]
    /// Base URL of the media stream, if applicable
    pub base_url: Option<String>,
    /// Available formats of the media stream, if applicable
    #[serde(rename = "availableFormats")]
    pub available_formats: Option<AvailableFormats>,
    #[serde(rename = "availableQualities")]
    /// Available qualities of the media stream, if applicable
    pub available_qualities: Option<HashMap<String, String>>,
    /// URL template for the media stream, if applicable
    #[serde(rename = "modelUrl")]
    pub model_url: Option<String>,
    /// Host of the media stream, if applicable
    pub host: Option<String>,
}

/// Represents track information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MediaDetails {
    /// Video tracks
    #[serde(deserialize_with = "hashmap_or_array_to_hashmap")]
    pub video: HashMap<String, VideoTrack>,
    /// Audio tracks
    #[serde(deserialize_with = "hashmap_or_array_to_hashmap")]
    pub audio: HashMap<String, AudioTrack>,
    /// Subtitle tracks
    #[serde(deserialize_with = "hashmap_or_array_to_hashmap")]
    pub subtitles: HashMap<String, SubtitleTrack>,
}

/// Represents video track information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct VideoTrack {
    /// Stream ID of the track
    pub stream: String,
    /// Language in plain text (e.g. "English", "French")
    pub lang: String,
    /// Language in iso_639 (e.g. "fre", "eng")
    pub lang_iso: String,
    /// Codec of the video (e.g. "h264", "divx")
    pub codec: String,
    /// Colorspace of the video (e.g. "yuv420p")
    pub colorspace: String,
    /// Width of the video (e.g. 1980)
    pub width: u64,
    /// Height of the video (e.g. 1080)
    pub height: u64,
}

/// Represents audio track information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AudioTrack {
    /// Stream ID of the track
    pub stream: String,
    /// Language in plain text (e.g. "English", "French")
    pub lang: String,
    /// Language in iso_639 (e.g. "fre", "eng")
    pub lang_iso: String,
    /// Codec of the audio (e.g. "aac", "mp3")
    pub codec: String,
    /// Audio sampling rate
    pub sampling: u64,
    /// Number of channels (e.g. 2, 5.1, 7.1)
    pub channels: f64,
}

/// Represents subtitle track information.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SubtitleTrack {
    /// Stream ID of the track
    pub stream: String,
    /// Language in plain text (e.g. "English", "French")
    pub lang: String,
    /// Language in iso_639 (e.g. "fre", "eng")
    pub lang_iso: String,
    /// Format of subtitles (e.g. "ASS" / "SRT")
    #[serde(rename = "type")]
    pub type_: String,
}

/// Represents available stream formats.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AvailableFormats {
    /// M3U8 Live Streaming format
    pub apple: String,
    /// MPD Live Streaming format
    pub dash: String,
    /// Live MP4
    #[serde(rename = "liveMP4")]
    pub live_mp4: String,
    /// Live H264 WebM
    #[serde(rename = "h264WebM")]
    pub h264_webm: String,
}

/// Enum representing the type of a media file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Show,
    Audio,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_transcode() -> Result<()> {
        let s = r#"
        {
            "apple": {
                "full": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/eng1\/none\/aac\/full.m3u8"
            },
            "dash": {
                "full": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/eng1\/none\/aac\/full.mpd"
            },
            "liveMP4": {
                "full": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/eng1\/none\/aac\/full.mp4"
            },
            "h264WebM": {
                "full": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/eng1\/none\/aac\/full.webm"
            }
        }
        "#;

        let transcode = serde_json::from_str::<Transcode>(s);

        assert!(transcode.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_media_info() -> Result<()> {
        let s = r#"
        {
            "filename": "Rick Astley - Never Gonna Give You Up",
            "hoster": "yt",
            "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
            "type": "audio",
            "season": null,
            "episode": null,
            "year": null,
            "duration": 212.140408,
            "bitrate": 128020,
            "size": 3394801,
            "details": {
                "video": [],
                "audio": {
                    "und1": {
                        "stream": "0:0",
                        "lang": "Unknown",
                        "lang_iso": "und",
                        "codec": "mp3",
                        "sampling": 44100,
                        "channels": 2
                    }
                },
                "subtitles": []
            },
            "backdrop_path": "https:\/\/lastfm.freetls.fastly.net\/i\/u\/ar0\/92c372883f05137bb7c6e9ec49afe403.jpg",
            "poster_path": "\/static\/images\/dummy_user1_48@2x.55894782493f.png",
            "audio_image": "\/static\/images\/dummy_user1_48@2x.55894782493f.png",
            "baseUrl": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/eng1\/none\/aac\/full",
            "availableFormats": {
                "apple": "m3u8",
                "dash": "mpd",
                "liveMP4": "mp4",
                "h264WebM": "webm"
            },
            "availableQualities": {
                "Original": "full"
            },
            "modelUrl": "https:\/\/example.com\/t\/ABCDEFGHIJKLMNO\/{audio}\/{subtitles}\/{audioCodec}\/{quality}.{format}",
            "host": "example.com"
        }
        "#;

        let media_info = serde_json::from_str::<MediaInfo>(s);

        assert!(media_info.is_ok());

        Ok(())
    }
}
