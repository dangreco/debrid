use std::collections::HashMap;

use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer,
};
use serde_json::Value;

pub fn hashmap_or_array_to_hashmap<'de, D, T>(deserializer: D) -> Result<HashMap<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::Object(obj) => {
            let mut hashmap = HashMap::new();
            for (key, value) in obj {
                match serde_json::from_value(value) {
                    Ok(v) => {
                        hashmap.insert(key, v);
                    }
                    Err(e) => {
                        return Err(serde::de::Error::custom(format!(
                            "Error deserializing object value: {}",
                            e
                        )))
                    }
                }
            }
            Ok(hashmap)
        }
        Value::Array(arr) => {
            let mut hashmap = HashMap::new();
            for (i, value) in arr.into_iter().enumerate() {
                match serde_json::from_value(value) {
                    Ok(v) => {
                        hashmap.insert(i.to_string(), v);
                    }
                    Err(e) => {
                        return Err(de::Error::custom(format!(
                            "Error deserializing array value: {}",
                            e
                        )))
                    }
                }
            }
            Ok(hashmap)
        }
        _ => Err(de::Error::custom("Expected an object or array")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStructOuter {
        #[serde(
            deserialize_with = "hashmap_or_array_to_hashmap",
        )]
        inner: HashMap<String, TestStructInner>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStructInner {
        a: i32,
        b: String,
    }

    #[test]
    fn test_deserialize_to_hashmap_object() {
        let json_str = r#"
        {
            "inner": {
                "key1": {
                    "a": 1,
                    "b": "value1"
                },
                "key2": {
                    "a": 2,
                    "b": "value2"
                }
            }
        }"#;

        let expected = TestStructOuter {
            inner: HashMap::from([
                (
                    "key1".to_string(),
                    TestStructInner {
                        a: 1,
                        b: "value1".to_string(),
                    },
                ),
                (
                    "key2".to_string(),
                    TestStructInner {
                        a: 2,
                        b: "value2".to_string(),
                    },
                ),
            ]),
        };

        let result: Result<TestStructOuter, _> = serde_json::from_str(json_str);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_to_hashmap_array() {
        let json_str = r#"
        {
            "inner": [
                {
                    "a": 1,
                    "b": "value1"
                },
                {
                    "a": 2,
                    "b": "value2"
                }
            ]
        }"#;

        let expected = TestStructOuter {
            inner: HashMap::from([
                (
                    "0".to_string(),
                    TestStructInner {
                        a: 1,
                        b: "value1".to_string(),
                    },
                ),
                (
                    "1".to_string(),
                    TestStructInner {
                        a: 2,
                        b: "value2".to_string(),
                    },
                ),
            ]),
        };

        let result: Result<TestStructOuter, _> = serde_json::from_str(json_str);
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_deserialize_to_hashmap_invalid_type() {
        let json_str = r#""not an object or array""#;
        let result: Result<TestStructOuter, _> = serde_json::from_str(json_str);
        assert!(result.is_err());
    }
}
