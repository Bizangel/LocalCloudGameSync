use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Revision {
    pub hash: String,
    pub timestamp: u64,
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
            .and_then(|dt| {
                Some(
                    dt.with_timezone(&Local)
                        .format("%H:%M:%S %a %e %b %Y [%Z]")
                        .to_string(),
                )
            })
            .unwrap_or_default();
        write!(f, "{} ({})", utctime, self.hash)
    }
}

impl Revision {
    pub fn serialize(&self) -> String {
        format!("{},{}", self.hash, self.timestamp)
    }

    pub fn deserialize(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err(format!("Empty revision")); // empty string → no revision
        }

        let mut parts = s.splitn(2, ',');

        let hash = parts
            .next()
            .ok_or_else(|| "Missing hash part".to_string())?
            .to_string();

        let timestamp_str = parts
            .next()
            .ok_or_else(|| "Missing timestamp part".to_string())?;

        let timestamp = timestamp_str
            .parse::<u64>()
            .map_err(|e| format!("Invalid timestamp: {}", e))?;

        Ok(Revision { hash, timestamp })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        assert_eq!(
            Revision {
                hash: "abcdef".to_string(),
                timestamp: 12000,
            }
            .serialize(),
            "abcdef|12000"
        )
    }

    #[test]
    fn deserialization() {
        let rev = Revision::deserialize("abcdef|12000").expect("Safe serialize");
        assert_eq!(rev.hash, "abcdef");
        assert_eq!(rev.timestamp, 12000);
    }
}
