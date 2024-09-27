/// Represents a RealDebrid error.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// Error message
    #[serde(rename = "error")]
    pub message: String,

    /// Error code
    #[serde(rename = "error_code")]
    pub code: i32,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::Error;

    #[test]
    fn test_deserialize() -> Result<()> {
        let s = r#"
        {
            "error": "Fair Usage Limit",
            "error_code": 36
        }
        "#;

        let error = serde_json::from_str::<Error>(s);

        assert!(error.is_ok());

        Ok(())
    }
}
