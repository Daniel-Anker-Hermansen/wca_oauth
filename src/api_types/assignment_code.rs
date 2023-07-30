use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub enum AssignmentCode {
    Competitor,
    Judge,
    Scrambler,
    Runner,
    DataEntry,
    Announcer,
    Other(Box<String>),
}

impl Serialize for AssignmentCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            AssignmentCode::Competitor => serializer.serialize_str("competitor"),
            AssignmentCode::Judge => serializer.serialize_str("staff-judge"),
            AssignmentCode::Scrambler => serializer.serialize_str("staff-scrambler"),
            AssignmentCode::Runner => serializer.serialize_str("staff-runner"),
            AssignmentCode::DataEntry => serializer.serialize_str("staff-dataentry"),
            AssignmentCode::Announcer => serializer.serialize_str("staff-announcer"),
            AssignmentCode::Other(s) => serializer.serialize_str(&s),
        }
    }
}

impl<'de> Deserialize<'de> for AssignmentCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "competitor" => AssignmentCode::Competitor,
            "staff-scrambler" => AssignmentCode::Scrambler,
            "staff-runner" => AssignmentCode::Runner,
            "staff-dataentry" => AssignmentCode::DataEntry,
            "staff-announcer" => AssignmentCode::Announcer,
            _ => AssignmentCode::Other(Box::new(s)),
        })
    }
}
