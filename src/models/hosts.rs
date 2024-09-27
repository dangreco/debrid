use std::collections::HashMap;

/// Represents a RealDebrid host.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Host {
    /// RealDebrid host ID
    pub id: String,
    /// Host's name
    pub name: String,
    /// Host's image (small, usually 16x16)
    pub image: String,
    /// Host's image (big, usually 100x100)
    pub image_big: Option<String>,
}

/// Represents status information of a RealDebrid host.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HostInfo {
    /// RealDebrid host ID
    pub id: String,
    /// Host's name
    pub name: String,
    /// Host's image (small, usually 16x16)
    pub image: String,
    /// Host's image (big, usually 100x100)
    pub image_big: Option<String>,
    /// Whether or not the host is supported
    #[serde(with = "crate::de::bool::zero_or_one")]
    pub supported: bool,
    /// Status of the host
    pub status: HostStatus,
    /// When the host's status was last checked
    pub check_time: String,
    /// Statuses of corresponding competitor hosts
    pub competitors_status: HashMap<String, CompetitorInfo>,
}

/// Represents the status information of a RealDebrid competitor host.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompetitorInfo {
    /// Status of the competitor host
    pub status: HostStatus,
    /// When the competitor hosts's status was last checked
    pub check_time: String,
}

/// Represents the current status of a RealDebrid host.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostStatus {
    Up,
    Down,
    Unsupported,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_deserialize_hosts() -> Result<()> {
        let s = r#"
        {
            "example0.com": {
                "id": "e0",
                "name": "example0",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100"
            },
            "example1.com": {
                "id": "e1",
                "name": "example1",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100"
            },
            "example2.com": {
                "id": "e2",
                "name": "example2",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100"
            }
        }
        "#;

        let hosts = serde_json::from_str::<HashMap<String, Host>>(s);

        assert!(hosts.is_ok());

        Ok(())
    }

    #[test]
    fn test_deserialize_hosts_status() -> Result<()> {
        let s = r#"
        {
            "example0.com": {
                "id": "e0",
                "name": "example0",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100",
                "supported": 1,
                "status": "up",
                "check_time": "2024-09-27T15:53:09.000Z",
                "competitors_status": {
                    "competitor0.com": {
                        "status": "up",
                        "check_time": "2024-09-27T15:53:12.000Z"
                    },
                    "competitor1.com": {
                        "status": "down",
                        "check_time": "2017-11-28T23:49:55.000Z"
                    },
                    "competitor2.com": {
                        "status": "down",
                        "check_time": "2024-09-27T15:53:09.000Z"
                    }
                }
            },
            "example1.com": {
                "id": "e1",
                "name": "example1",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100",
                "supported": 1,
                "status": "up",
                "check_time": "2024-09-27T15:53:14.000Z",
                "competitors_status": {
                    "competitor3.com": {
                        "status": "up",
                        "check_time": "2024-09-27T15:53:15.000Z"
                    },
                    "competitor4.com": {
                        "status": "down",
                        "check_time": "2017-11-28T23:49:59.000Z"
                    },
                    "competitor5.com": {
                        "status": "down",
                        "check_time": "2024-09-27T15:53:12.000Z"
                    }
                }
            },
            "example2.com": {
                "id": "e2",
                "name": "example2",
                "image": "https://picsum.photos/16",
                "image_big": "https://picsum.photos/100",
                "supported": 0,
                "status": "down",
                "check_time": "2024-09-27T15:53:15.000Z",
                "competitors_status": {
                    "competitor6.com": {
                        "status": "down",
                        "check_time": "2024-09-27T15:54:15.000Z"
                    },
                    "competitor7.com": {
                        "status": "down",
                        "check_time": "2017-11-28T23:50:00.000Z"
                    },
                    "competitor8.com": {
                        "status": "down",
                        "check_time": "2024-09-27T15:53:15.000Z"
                    }
                }
            }
        }
        "#;

        let hosts = serde_json::from_str::<HashMap<String, HostInfo>>(s);

        assert!(hosts.is_ok());

        Ok(())
    }
}
