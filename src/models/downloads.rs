/// Represents a download from RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Download {
    /// Download ID
    pub id: String,
    /// Filename
    pub filename: String,
    /// MIME type of the file, guessed by the file extension
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Bytes, 0 if unknown
    pub filesize: u64,
    /// Original link
    pub link: String,
    /// Host main domain
    pub host: String,
    /// Max chunks allowed
    pub chunks: u64,
    /// Generated link
    pub download: String,
    /// Date download was generated
    pub generated: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_downloads() -> Result<()> {
        let s = r#"
        [
            {
                "id": "ABCDEFGHIJKLMNO",
                "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp3",
                "mimeType": "audio\/mpeg",
                "filesize": 0,
                "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
                "host": "youtube.com",
                "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
                "chunks": 32,
                "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp3",
                "streamable": 1,
                "generated": "2024-09-27T08:29:36.000Z",
                "type": "mp3"
            },
            {
                "id": "ABCDEFGHIJKLMNO",
                "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                "mimeType": "video\/mp4",
                "filesize": 0,
                "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
                "host": "youtube.com",
                "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
                "chunks": 32,
                "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                "streamable": 1,
                "generated": "2024-09-27T08:29:36.000Z",
                "type": "426x240"
            },
            {
                "id": "ABCDEFGHIJKLMNO",
                "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                "mimeType": "video\/mp4",
                "filesize": 0,
                "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
                "host": "youtube.com",
                "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
                "chunks": 32,
                "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                "streamable": 1,
                "generated": "2024-09-27T08:29:36.000Z",
                "type": "640x360"
            },
            {
                "id": "ABCDEFGHIJKLMNO",
                "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                "mimeType": "video\/mp4",
                "filesize": 0,
                "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
                "host": "youtube.com",
                "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
                "chunks": 32,
                "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                "streamable": 1,
                "generated": "2024-09-27T08:29:36.000Z",
                "type": "854x480"
            },
            {
                "id": "ABCDEFGHIJKLMNO",
                "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                "mimeType": "video\/mp4",
                "filesize": 0,
                "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
                "host": "youtube.com",
                "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
                "chunks": 32,
                "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                "streamable": 1,
                "generated": "2024-09-27T08:29:36.000Z",
                "type": "1280x720"
            }
        ]
        "#;

        let downloads = serde_json::from_str::<Vec<Download>>(s);

        assert!(downloads.is_ok());

        Ok(())
    }
}
