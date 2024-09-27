use std::collections::HashMap;

/// Represents a RealDebrid user's settings.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// Possible "download_port" value to update settings
    pub download_ports: Vec<String>,
    /// Current user download port
    pub download_port: String,
    /// Possible "locale" value to update settings
    pub locales: HashMap<String, String>,
    /// Current user locale
    pub locale: String,
    /// Possible "streaming_quality" value to update settings
    pub streaming_qualities: Vec<String>,
    /// Current user streaming quality
    pub streaming_quality: String,
    /// Current user streaming quality on mobile devices
    pub mobile_streaming_quality: String,
    /// Possible "streaming_language_preference" value to update settings
    pub streaming_languages: HashMap<String, String>,
    /// Current user streaming language preference
    pub streaming_language_preference: String,
    /// Possible "streaming_cast_audio_preference" value to update settings
    pub streaming_cast_audio: Vec<String>,
    /// Current user audio preference on Google Cast devices
    pub streaming_cast_audio_preference: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_settings() -> Result<()> {
        let s = r#"
        {
            "download_ports": [
                "normal",
                "secured"
            ],
            "download_port": "secured",
            "download_protocols": [
                "IPv4+IPv6",
                "IPv4",
                "IPv6"
            ],
            "download_protocol": "IPv4",
            "locales": {
                "fr": "Français (France)",
                "en": "English (US)",
                "es": "Español",
                "it": "Italiano",
                "de": "Deutsch",
                "ar": "العربية",
                "pt": "Português",
                "ptb": "Português (Brasil)",
                "tr": "Türkçe",
                "nl": "Dutch",
                "ir": "Persian (Farsi)",
                "cnt": "繁體中文",
                "cns": "简体中文",
                "pl": "Polish",
                "id": "Indonesian",
                "hr": "Croatian",
                "ru": "Russian",
                "ro": "Romanian"
            },
            "locale": "en",
            "streaming_qualities": [
                "original",
                "high",
                "medium",
                "low"
            ],
            "streaming_quality": "original",
            "mobile_streaming_quality": "original",
            "streaming_languages": {
                "fre": "Français",
                "eng": "English",
                "ita": "Italiano",
                "esp": "Español"
            },
            "streaming_language_preference": "eng",
            "streaming_cast_audio": [
                "aac",
                "dolby"
            ],
            "streaming_cast_audio_preference": "aac"
        }
        "#;

        let settings = serde_json::from_str::<Settings>(s);

        assert!(settings.is_ok());

        Ok(())
    }
}
