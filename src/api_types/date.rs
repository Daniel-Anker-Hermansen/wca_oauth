pub use chrono::Datelike;
use chrono::NaiveDate;

/// Represents a date. This is the local date.
pub type Date = NaiveDate;

#[cfg(test)]
mod test {
    use super::Date;
    use chrono::Datelike;

    #[test]
    fn test_serialization() {
        let date = "\"2023-03-10\"";
        let serialized = serde_json::from_str::<Date>(date).unwrap();
        let deserialized = serde_json::to_string(&serialized).unwrap();
        assert_eq!(serialized.day(), 10);
        assert_eq!(serialized.month(), 3);
        assert_eq!(serialized.year(), 2023);
        assert_eq!(date, deserialized);
    }
}
