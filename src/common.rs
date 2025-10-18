use base64::{Engine as _, engine::general_purpose};
use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Revision {
    pub hash: String,
    pub timestamp: u64,
    pub author: String,
}

impl PartialEq for Revision {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl fmt::Display for Revision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timestamp_as_int = i64::try_from(self.timestamp).unwrap_or_default();
        let utctime = DateTime::from_timestamp_secs(timestamp_as_int)
            .map(|dt| {
                dt.with_timezone(&Local)
                    .format("%H:%M:%S %a %e %b %Y [%Z]")
                    .to_string()
            })
            .unwrap_or_default();

        write!(f, "{} ({}, by {})", utctime, self.hash, self.author)
    }
}

impl Revision {
    pub fn serialize(&self) -> String {
        let author_encoded = general_purpose::STANDARD.encode(&self.author);
        format!("{},{},{}", self.hash, self.timestamp, author_encoded)
    }

    pub fn deserialize(s: &str) -> Result<Self, String> {
        let mut parts = s.splitn(3, ',');
        let hash = parts.next().ok_or("Missing hash part")?.to_string();
        let timestamp_str = parts.next().ok_or("Missing timestamp part")?;
        let timestamp = timestamp_str
            .parse::<u64>()
            .map_err(|e| format!("Invalid timestamp: {}", e))?;

        let author_encoded = parts.next().ok_or("Missing author part")?;

        let author_bytes = general_purpose::STANDARD
            .decode(author_encoded)
            .map_err(|e| format!("Invalid base64 author: {}\n {}", author_encoded, e))?;
        let author = String::from_utf8(author_bytes)
            .map_err(|e| format!("Invalid UTF-8 in author: {}", e))?;

        Ok(Revision {
            hash,
            timestamp,
            author,
        })
    }

    pub fn time_display_str(&self) -> String {
        let timestamp_as_int = i64::try_from(self.timestamp).unwrap_or_default();
        DateTime::from_timestamp_secs(timestamp_as_int)
            .map(|dt| {
                dt.with_timezone(&Local)
                    .format("%a %e %b %Y at %H:%M:%S")
                    .to_string()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_roundtrip() {
        let rev = Revision {
            hash: "abcdef".to_string(),
            timestamp: 12000,
            author: "Jane Doe".to_string(),
        };
        let serialized = rev.serialize();
        let deserialized = Revision::deserialize(&serialized).unwrap();

        assert_eq!(rev.hash, deserialized.hash);
        assert_eq!(rev.timestamp, deserialized.timestamp);
        assert_eq!(rev.author, deserialized.author);
    }
}
