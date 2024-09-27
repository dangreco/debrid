#[allow(unused)]
pub mod zero_or_one {
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if *value {
            serializer.serialize_u8(1)
        } else {
            serializer.serialize_u8(0)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match deserializer.deserialize_u8(Visitor)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(de::Error::custom(format!(
                "invalid value: {}, expected 0 or 1",
                other
            ))),
        }
    }

    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = u8;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an integer between 0 and 1")
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if value > 1 {
                return Err(E::custom(format!("integer out of range: {}", value)));
            }
            Ok(value as u8)
        }
    }
}

#[allow(unused)]
pub mod opt_zero_or_one {
    use serde::{de, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(true) => serializer.serialize_u8(1),
            Some(false) => serializer.serialize_u8(0),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Option<bool>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an integer between 0 and 1, or null")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Some(false)),
                    1 => Ok(Some(true)),
                    _ => Err(E::custom(format!(
                        "invalid value: {}, expected 0 or 1",
                        value
                    ))),
                }
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }
        }

        deserializer.deserialize_option(Visitor)
    }
}
