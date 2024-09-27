/// Represents an unrestrict/downloadable check on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Check {
    /// Host main domain
    pub host: String,
    /// Host icon, if applicable
    pub host_icon: Option<String>,
    /// Host icon, if applicable
    pub host_icon_big: Option<String>,
    /// Original file link
    pub link: String,
    /// Original file name
    pub filename: String,
    /// Original file size in bytes, 0 if unknown
    pub filesize: u64,
    /// Whether or not the file is supported
    #[serde(with = "crate::de::bool::zero_or_one")]
    pub supported: bool,
}

/// Represents an unrestricted link on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Link {
    /// RealDebrid link ID
    pub id: String,
    /// Original filename
    pub filename: String,
    /// MIME type of the file, if applicable
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    /// Original file size in bytes, 0 if unknown
    pub filesize: u64,
    /// Original file link
    pub link: String,
    /// Host main domain
    pub host: String,
    /// Max chunks allowed
    pub chunks: u64,
    /// Disable / enable CRC check
    #[serde(with = "crate::de::bool::zero_or_one")]
    pub crc: bool,
    /// Generated download link
    pub download: String,
    /// Whether or not the file is streamable
    #[serde(with = "crate::de::bool::zero_or_one")]
    pub streamable: bool,
    /// Type of the file (in general, its quality), if applicable
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Quality of the file, if applicable
    pub quality: Option<String>,
    /// Alternative download links, if applicable
    pub alternative: Option<Vec<AlternativeLink>>,
}

/// Represents an alternate download link on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlternativeLink {
    /// RealDebrid link ID
    pub id: String,
    /// Original filename
    pub filename: String,
    /// MIME type of the file, if applicable
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    /// Generated download link
    pub download: String,
    /// Type of the file (in general, its quality), if applicable
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Quality of the file, if applicable
    pub quality: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_check() -> Result<()> {
        let s = r#"
        {
            "host": "youtube.com",
            "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
            "host_icon_big": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/100_100\/youtube.png",
            "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
            "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video)|-|1920x1080,1280x720,854x480,640x360,426x240",
            "filesize": 0,
            "supported": 1
        }
        "#;

        let check = serde_json::from_str::<Check>(s);

        assert!(check.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_alternative_link() -> Result<()> {
        let s = r#"
        {
            "id": "ABCDEFGHIJKLMNO",
            "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
            "mimeType": "video\/mp4",
            "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
            "quality": "HQ"
        }
        "#;

        let link = serde_json::from_str::<AlternativeLink>(s);

        assert!(link.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_link_single() -> Result<()> {
        let s = r#"
        {
            "id": "ABCDEFGHIJKLMNO",
            "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
            "mimeType": "video\/mp4",
            "filesize": 0,
            "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
            "host": "youtube.com",
            "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
            "chunks": 32,
            "crc": 1,
            "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
            "streamable": 1,
            "quality": "1080p"
        }
        "#;

        let link = serde_json::from_str::<Link>(s);

        assert!(link.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_link_multiple() -> Result<()> {
        let s = r#"
        {
            "id": "ABCDEFGHIJKLMNO",
            "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
            "mimeType": "video\/mp4",
            "filesize": 0,
            "link": "https:\/\/www.youtube.com\/watch?v=dQw4w9WgXcQ",
            "host": "youtube.com",
            "host_icon": "https:\/\/fcdn.real-debrid.com\/0830\/images\/hosters\/youtube.png",
            "chunks": 32,
            "crc": 1,
            "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
            "streamable": 1,
            "quality": "1080p",
            "alternative": [
                {
                    "id": "ABCDEFGHIJKLMNO",
                    "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                    "mimeType": "video\/mp4",
                    "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                    "quality": "720p"
                },
                {
                    "id": "ABCDEFGHIJKLMNO",
                    "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                    "mimeType": "video\/mp4",
                    "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                    "quality": "HQ"
                },
                {
                    "id": "ABCDEFGHIJKLMNO",
                    "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                    "mimeType": "video\/mp4",
                    "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                    "quality": "SD"
                },
                {
                    "id": "ABCDEFGHIJKLMNO",
                    "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp4",
                    "mimeType": "video\/mp4",
                    "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp4",
                    "quality": "240p"
                },
                {
                    "id": "ABCDEFGHIJKLMNO",
                    "filename": "Rick Astley - Never Gonna Give You Up (Official Music Video).mp3",
                    "mimeType": "audio\/mpeg",
                    "download": "https:\/\/example.com\/d\/ABCDEFGHIJKLMNO\/Rick%20Astley%20-%20Never%20Gonna%20Give%20You%20Up%20%28Official%20Music%20Video%29.mp3"
                }
            ]
        }
        "#;

        let link = serde_json::from_str::<AlternativeLink>(s);

        assert!(link.is_ok());

        Ok(())
    }
}
