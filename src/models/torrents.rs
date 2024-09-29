use std::collections::HashMap;

/// Represents a RealDebrid torrent.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Torrent {
    /// RealDebrid torrent ID
    pub id: String,
    /// Filename
    pub filename: String,
    /// SHA1 Hash of the torrent
    pub hash: String,
    /// Size of selected files only
    pub bytes: u64,
    /// Host main domain
    pub host: String,
    /// Split size of links
    pub split: u64,
    /// Progress of the torrent, from 0-100
    pub progress: u8,
    /// Current status of the torrent
    pub status: TorrentStatus,
    /// Date at which the torrent was added to RealDebrid
    pub added: String,
    /// Links for the torrent
    pub links: Vec<String>,
    /// Only present when finished; the date at which the torrent completed
    pub ended: Option<String>,
    /// Only present when downloading, compressing or uploading; the speed of the torrent
    pub speed: Option<u64>,
    /// Only present when downloading or converting magnet; the number of seeders of the torrent
    pub seeders: Option<u64>,
}

/// Enum representing the current status of a RealDebrid torrent.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum TorrentStatus {
    MagnetError,
    MagnetConversion,
    WaitingFilesSelection,
    Queued,
    Downloading,
    Downloaded,
    Error,
    Virus,
    Compressing,
    Uploading,
    Dead,
}

/// Represents information of a RealDebrid torrent.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TorrentInfo {
    /// RealDebrid torrent ID
    pub id: String,
    /// Filename
    pub filename: String,
    /// Original name of the torrent
    pub original_filename: String,
    /// SHA1 Hash of the torrent
    pub hash: String,
    /// Size of selected files only
    pub bytes: u64,
    /// Total size of the torrent
    pub original_bytes: u64,
    /// Host main domain
    pub host: String,
    /// Split size of links
    pub split: u64,
    /// Progress of the torrent, from 0-100
    pub progress: u8,
    /// Current status of the torrent
    pub status: TorrentStatus,
    /// Date at which the torrent was added to RealDebrid
    pub added: String,
    /// Files included in the torrent
    pub files: Vec<TorrentFile>,
    /// Links for the torrent
    pub links: Vec<String>,
    /// Only present when finished; the date at which the torrent completed
    pub ended: Option<String>,
    /// Only present when downloading, compressing or uploading; the speed of the torrent
    pub speed: Option<u64>,
    /// Only present when downloading or converting magnet; the number of seeders of the torrent
    pub seeders: Option<u64>,
}

/// Represents a file of a RealDebrid torrent.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TorrentFile {
    /// RealDebrid torrent file ID
    pub id: u64,
    /// Path to the file inside the torrent, starting with "/"
    pub path: String,
    /// Size of the file in bytes
    pub bytes: u64,
    /// Whether or not the file has been selected
    #[serde(with = "crate::de::bool::zero_or_one")]
    pub selected: bool,
}

/// Represents the instant availability of a RealDebrid torrent.
/// Hoster x File ID variants x File ID x File
pub type InstantAvailability = HashMap<String, Vec<HashMap<String, InstantlyAvailableFile>>>;

/// Represents an instantly available file on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InstantlyAvailableFile {
    /// Filename
    pub filename: String,
    /// Size of the file in bytes
    pub filesize: u64,
}

/// Represents the active torrent count on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ActiveCount {
    /// Number of currently active torrents
    pub nb: u64,
    /// Maximum number of active torrents you can have
    pub limit: u64,
    /// List of active torrents
    pub list: Option<Vec<String>>,
}

/// Represents an availble torrent host on RealDebrid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AvailableHost {
    /// Host main domain
    pub host: String,
    /// Max split size possible
    pub max_file_size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AddedTorrent {
    /// RealDebrid torrent ID
    pub id: String,
    /// URL of the created ressource
    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_torrents() -> Result<()> {
        let s = r#"
        [
            {
                "id": "ABCDEFGHIJKLM",
                "filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
                "hash": "c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab",
                "bytes": 183567938,
                "host": "example.com",
                "split": 2000,
                "progress": 100,
                "status": "downloaded",
                "added": "2024-09-27T09:25:11.000Z",
                "links": [
                    "https:\/\/example.com\/d\/ABCDEFGHIJKLMNOP"
                ],
                "ended": "2024-09-22T06:15:21.000Z"
            },
            {
                "id": "ABCDEFGHIJKLM",
                "filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
                "hash": "c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab",
                "bytes": 183567938,
                "host": "example.com",
                "split": 2000,
                "progress": 0,
                "status": "downloading",
                "added": "2024-09-27T09:23:41.000Z",
                "links": [],
                "speed": 0,
                "seeders": 0
            }
        ]
        "#;

        let torrents = serde_json::from_str::<Vec<Torrent>>(s);

        assert!(torrents.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_torrent_info() -> Result<()> {
        let s = r#"
        {
            "id": "ABCDEFGHIJKLM",
            "filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
            "original_filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
            "hash": "c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab",
            "bytes": 183567938,
            "original_bytes": 183567938,
            "host": "example.com",
            "split": 2000,
            "progress": 100,
            "status": "downloaded",
            "added": "2024-09-27T09:25:11.000Z",
            "files": [
                {
                    "id": 1,
                    "path": "\/README.txt",
                    "bytes": 71,
                    "selected": 0
                },
                {
                    "id": 2,
                    "path": "\/Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
                    "bytes": 253332230,
                    "selected": 1
                },
                {
                    "id": 3,
                    "path": "\/source.txt",
                    "bytes": 48,
                    "selected": 0
                }
            ],
            "links": [
                "https:\/\/example.com\/d\/ABCDEFGHIJKLMNOP"
            ],
            "ended": "2024-09-22T06:15:21.000Z"
        }
        "#;

        let info = serde_json::from_str::<TorrentInfo>(s);

        assert!(info.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_torrent_instant_availability() -> Result<()> {
        let s = r#"
        {
            "c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab": {
                "rd": [
                    {
                        "1": {
                            "filename": "README.txt",
                            "filesize": 71
                        }
                    },
                    {
                        "1": {
                            "filename": "README.txt",
                            "filesize": 71
                        },
                        "2": {
                            "filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
                            "filesize": 183567938
                        },
                        "3": {
                            "filename": "source.txt",
                            "filesize": 48
                        }
                    },
                    {
                        "2": {
                            "filename": "Big.Buck.Bunny.BDRip.XviD-MEDiC.(www.USABIT.com).avi",
                            "filesize": 183567938
                        }
                    }
                ]
            }
        }
        "#;

        let availability = serde_json::from_str::<HashMap<String, InstantAvailability>>(s);

        assert!(availability.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_torrents_active_count() -> Result<()> {
        let s = r#"
        {
            "nb": 1,
            "limit": 42,
            "list": [
                "c39fe3eefbdb62da9c27eb6398ff4a7d2e26e7ab"
            ]
        }
        "#;

        let count = serde_json::from_str::<ActiveCount>(s);

        assert!(count.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_torrents_available_hosts() -> Result<()> {
        let s = r#"
        [
            {
                "host": "example.com",
                "max_file_size": 2000
            }
        ]
        "#;

        let hosts = serde_json::from_str::<Vec<AvailableHost>>(s);

        assert!(hosts.is_ok());

        Ok(())
    }
}
