use std::collections::HashMap;

/// Enum representing traffic information for limited RealDebrid hosters.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Traffic {
    /// Traffic limited by links
    Links {
        /// Available links to use
        left: u64,
        /// Links unrestricted
        links: u64,
        /// Maximum amount of links that can be unrestricted, if applicable
        limit: Option<u64>,
        /// Additional links the user may have bought, if applicable
        extra: Option<u64>,
        /// Reset frequency of the limit, if applicable
        reset: Option<Reset>,
    },
    /// Traffic limited by bandwidth, in gigabytes
    Gigabytes {
        /// Available bytes to use
        left: u64,
        /// Bytes downloaded
        bytes: Option<u64>,
        /// Maximum amount of bytes that can be downloaded, if applicable
        limit: Option<u64>,
        /// Additional traffic the user may have bought, if applicable
        extra: Option<u64>,
        /// Reset frequency of the limit, if applicable
        reset: Option<Reset>,
    },
    /// Traffic limited by bandwidth, in bytes
    Bytes {
        /// Available Bytes to use
        left: u64,
        /// Bytes downloaded
        bytes: Option<u64>,
        /// Maximum amount of bytes that can be downloaded, if applicable
        limit: Option<u64>,
        /// Additional traffic the user may have bought, if applicable
        extra: Option<u64>,
        /// Reset frequency of the limit, if applicable
        reset: Option<Reset>,
    },
}

/// Enum representing the reset frequency of a hoster limit.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reset {
    Daily,
    Weekly,
    Monthly,
}

/// Traffic details for a hoster for a day.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Detail {
    /// Host main domain x (host, bytes downloaded on host)
    pub host: HashMap<String, u64>,
    /// Total downloaded (in bytes) this day
    pub bytes: u64,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_traffic() -> Result<()> {
        let s = r#"
        {
            "example0.com": {
                "left": 5368709120,
                "bytes": 0,
                "links": 0,
                "limit": 5,
                "type": "gigabytes",
                "extra": 0,
                "reset": "daily"
            },
            "example1.com": {
                "left": 5,
                "bytes": 0,
                "links": 0,
                "limit": 5,
                "type": "links",
                "extra": 0,
                "reset": "daily"
            },
            "example3.com": {
                "left": 5368709120,
                "bytes": 0,
                "links": 0,
                "limit": 5,
                "type": "gigabytes",
                "extra": 0,
                "reset": "daily"
            },
            "example4.com": {
                "left": 0,
                "type": "bytes"
            }
        }
        "#;

        let traffic = serde_json::from_str::<HashMap<String, Traffic>>(s);

        assert!(traffic.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_traffic_details() -> Result<()> {
        let s = r#"
        {
            "2024-09-27": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-26": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-25": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-24": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-23": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-21": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            },
            "2024-09-20": {
                "host": {
                    "example.com": 123456789012
                },
                "bytes": 123456789012
            }
        }
        "#;

        let details = serde_json::from_str::<HashMap<String, Detail>>(s);

        assert!(details.is_ok());

        Ok(())
    }
}
