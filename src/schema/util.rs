pub mod int {
    use std::num::ParseIntError;
    use serde::de::{self, Deserialize, Deserializer};
    use serde::ser::Serializer;

    pub fn parse(s: &str) -> Result<u32, ParseIntError> {
        fn matches(s: &str, base: u32, p: &[&str]) -> Option<(u32, usize)> {
            p.iter()
                .find(|&p| s.starts_with(p))
                .map(|p| (base, p.len()))
        }

        // Don't use "0" as a prefix for octal literals, it appears in signal indexes.
        let (base, skip) = matches(s, 16, &["0x", "0X"])
            .or_else(|| matches(s, 8, &["0o", "0O"]))
            .or_else(|| matches(s, 2, &["0b", "0B"]))
            .unwrap_or((10, 0));

        let (_, s) = s.split_at(skip);
        u32::from_str_radix(s, base)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
        where D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        parse(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S>(value: &u32, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_str(value)
    }
}

pub mod opt_int {
    use super::int;
    use serde::de::{self, Deserialize, Deserializer};
    use serde::ser::Serializer;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
        where D: Deserializer<'de>,
    {
        let s: Option<String> = Deserialize::deserialize(deserializer)?;
        if let Some(ref s) = s {
            int::parse(s).map(|int| Some(int)).map_err(de::Error::custom)
        } else {
            Ok(None)
        }
    }

    pub fn serialize<S>(value: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        if let Some(ref int) = *value {
            int::serialize(int, serializer)
        } else {
            serializer.serialize_none()
        }
    }
}
