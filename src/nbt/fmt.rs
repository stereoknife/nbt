use super::types::{Tag, Payload, ID};

impl ToString for Tag {
    fn to_string(&self) -> String {
        let id = self.id().to_string();
        let payload = self.payload();
        if let Option::Some(p) = payload {
            format!("{}: {}", id, p.to_string())
        } else {
            id
        }
    }
}

impl ToString for Payload {
    fn to_string(&self) -> String {
        match self {
            Payload::Byte(p) => p.to_string(),
            Payload::Short(p) => p.to_string(),
            Payload::Int(p) => p.to_string(),
            Payload::Long(p) => p.to_string(),
            Payload::Float(p) => p.to_string(),
            Payload::Double(p) => p.to_string(),
            Payload::String(p) => p.clone(),
            Payload::List(p) => format!("[{}]", p.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")),
            Payload::Compound(p) => format!("[{}]", p.iter().map(|(_, t)| t.to_string()).collect::<Vec<_>>().join(", ")),
            Payload::ByteArray(p) => format!("{:?}", p),
            Payload::IntArray(p) => format!("{:?}", p),
            Payload::LongArray(p) => format!("{:?}", p),
        }
    }
}

impl ToString for ID {
    fn to_string(&self) -> String {
        match self {
            ID::End => String::from("END"),
            ID::Byte => String::from("BYTE"),
            ID::Short => String::from("SHORT"),
            ID::Int => String::from("INT"),
            ID::Long => String::from("LONG"),
            ID::Float => String::from("FLOAT"),
            ID::Double => String::from("DOUBLE"),
            ID::String => String::from("STRING"),
            ID::List => String::from("LIST"),
            ID::Compound => String::from("COMPOUND"),
            ID::ByteArray => String::from("BYTE ARRAY"),
            ID::IntArray => String::from("INT ARRAY"),
            ID::LongArray => String::from("LONG ARRAY"),
        }
    }
}