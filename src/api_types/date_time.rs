use std::ops::{Deref, DerefMut};
use std::fmt::Debug;

use chrono::{NaiveDateTime, ParseError};
use serde::{de::Visitor, Deserialize, Serialize};

/// Represents a time in zulu time.
#[derive(PartialEq, Eq)]
pub struct DateTime(NaiveDateTime);

impl Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for DateTime {
    type Target = NaiveDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deserialize(date: &str) -> Result<DateTime, ParseError> {
    NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SZ").map(|z| DateTime(z))
}

fn serialize(date: &DateTime) -> String {
    date.format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(&serialize(self))
    }
}

struct DTVisitor;

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        deserializer.deserialize_str(DTVisitor)
    }
}

impl<'de> Visitor<'de> for DTVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("date formatted like \"yyyy-mm-ddThh:mm:ssZ\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error {
        deserialize(v).map_err(E::custom)
    }
}

#[cfg(test)]
mod test {
    use crate::api_types::{
        date_time::{deserialize, serialize},
        DateTime,
    };

    use chrono::{Datelike, Timelike};

    #[test]
    fn test_serialize() {
        let date = "2023-03-10T15:30:00Z";
        let deserialized = deserialize(&date).unwrap();
        assert_eq!(deserialized.year(), 2023);
        let serialized = serialize(&deserialized);
        assert_eq!(date, &serialized);
    }

    #[test]
    fn test_serialization() {
        let date = "\"2023-03-10T15:30:00Z\"";
        let deserialized: DateTime = serde_json::from_str(date).unwrap();
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(deserialized.day(), 10);
        assert_eq!(deserialized.month(), 3);
        assert_eq!(deserialized.year(), 2023);
        assert_eq!(deserialized.hour(), 15);
        assert_eq!(deserialized.minute(), 30);
        assert_eq!(deserialized.second(), 0);
        assert_eq!(date, &serialized);
    }
}
