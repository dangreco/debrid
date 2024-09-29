/// Represents a RealDebrid user.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct User {
    /// RealDebrid user ID
    pub id: i64,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Fidelity points
    pub points: i64,
    /// User language
    pub locale: String,
    /// User avatar URL
    pub avatar: String,
    /// User type
    #[serde(rename = "type")]
    pub type_: Type,
    /// Seconds left as a Premium user
    pub premium: i64,
    /// Premium user expiration date
    pub expiration: String,
}

/// Enum representing the type of user.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Free,
    Premium,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::User;

    #[test]
    fn test_deserialize() -> Result<()> {
        let s = r#"
        {
            "id": 12345678,
            "username": "foobar",
            "email": "foo******@example.com",
            "points": 800,
            "locale": "en",
            "avatar": "https:\/\/fcdn.real-debrid.com\/images\/forum\/empty.png",
            "type": "premium",
            "premium": 1234567,
            "expiration": "2030-01-01T00:00:00.000Z"
        }
        "#;

        let user = serde_json::from_str::<User>(s);

        assert!(user.is_ok());

        Ok(())
    }
}
