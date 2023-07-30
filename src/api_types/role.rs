use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Delegate,
    TraineeDelegate,
    Organizer,
    Other(Box<String>),
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        match self {
            Role::Delegate => serializer.serialize_str("delegate"),
            Role::TraineeDelegate => serializer.serialize_str("trainee-delegate"),
            Role::Organizer => serializer.serialize_str("organizer"),
            Role::Other(role) => serializer.serialize_str(&role),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "delegate" => Role::Delegate,
            "trainee-delegate" => Role::TraineeDelegate,
            "organizer" => Role::Organizer,
            _ => Role::Other(Box::new(s)),
        })
    }
}
