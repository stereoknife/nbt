use std::any::Any;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tag {
    End,
    Tag (String, Payload),
}

impl Tag {
    pub fn end() -> Tag {
        Tag::End
    }

    pub fn new(name: String, payload: Payload) -> Tag {
        Tag::Tag(name, payload)
    }

    pub fn anon(&self) -> bool {
        match self {
            Self::End => true,
            Self::Tag(name, _) => name.is_empty()
        }
    }

    pub fn payload(&self) -> Option<&Payload> {
        match self {
            Self::End => None,
            Self::Tag(_, payload) => Some(&payload)
        }
    }

    pub fn name(&self) -> Option<&String> {
        match self {
            Self::End => None,
            Self::Tag(name, _) => if name.is_empty() { None } else { Some(name) }
        }
    }

    pub fn id(&self) -> ID {
        match self {
            Self::End => ID::End,
            Self::Tag(_, payload) => payload.to_id()
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            Self::End => None,
            Self::Tag(_, payload) => payload.len()
        }
    }
}

#[derive(Debug)]
pub enum Payload {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Payload>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

impl Payload {
    fn to_id(&self) -> ID {
        match self {
            Self::Byte(_) => ID::Byte,
            Self::Short(_) => ID::Short,
            Self::Int(_) => ID::Int,
            Self::Long(_) => ID::Long,
            Self::Float(_) => ID::Float,
            Self::Double(_) => ID::Double,
            Self::ByteArray(_) => ID::ByteArray,
            Self::String(_) => ID::String,
            Self::List(_) => ID::List,
            Self::Compound(_) => ID::Compound,
            Self::IntArray(_) => ID::IntArray,
            Self::LongArray(_) => ID::LongArray
        }
    }

    pub fn to_raw_id(&self) -> i8 {
        self.to_id() as i8
    }

    pub fn to_i8(&self) -> Option<i8> {
        match self {
            Self::Byte(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_i16(&self) -> Option<i16> {
        match self {
            Self::Short(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Self::Int(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Self::Long(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_f32(&self) -> Option<f32> {
        match self {
            Self::Float(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Self::Double(p) => Some(*p),
            _ => None
        }
    }

    pub fn to_vec(&self) -> Option<&Vec<Payload>> {
        match self {
            Self::List(p) => Some(p),
            _ => None
        }
    }

    pub fn to_u8_vec(&self) -> Option<&Vec<u8>> {
        match self {
            Self::ByteArray(p) => Some(p),
            _ => None
        }
    }

    pub fn to_i32_vec(&self) -> Option<&Vec<i32>> {
        match self {
            Self::IntArray(p) => Some(p),
            _ => None
        }
    }

    pub fn to_i64_vec(&self) -> Option<&Vec<i64>> {
        match self {
            Self::LongArray(p) => Some(p),
            _ => None
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        match self {
            Self::String(p) => Some(p.as_str()),
            _ => None
        }
    }

    pub fn to_map(&self) -> Option<&HashMap<String, Tag>> {
        match self {
            Self::Compound(p) => Some(p),
            _ => None
        }
    }

    fn len(&self) -> Option<usize> {
        match self {
            Self::List(p) => Some(p.len()),
            Self::String(p) => Some(p.len()),
            Self::IntArray(p) => Some(p.len()),
            Self::LongArray(p) => Some(p.len()),
            Self::Compound(p) => Some(p.len()),
            _ => None
        }
    }

    pub fn to_type(&'static self) -> Option<&dyn Any> {
        match self {
            Self::Byte(p) => Some(p),
            Self::Short(p) => Some(p),
            Self::Int(p) => Some(p),
            Self::Long(p) => Some(p),
            Self::Float(p) => Some(p),
            Self::Double(p) => Some(p),
            Self::ByteArray(p) => Some(&*p),
            Self::String(p) => Some(p),
            Self::List(p) => Some(&*p),
            // Self::Compound(p) => Some(&*self.to_map()?),
            Self::IntArray(p) => Some(&*p),
            Self::LongArray(p) => Some(&*p),
            _ => None
        }
    }

    pub fn to_type_rec(&'static self) -> Option<&dyn Any> {
        match self {
            Self::Byte(p) => Some(p),
            Self::Short(p) => Some(p),
            Self::Int(p) => Some(p),
            Self::Long(p) => Some(p),
            Self::Float(p) => Some(p),
            Self::Double(p) => Some(p),
            Self::ByteArray(p) => Some(&*p),
            Self::String(p) => Some(p),
            Self::IntArray(p) => Some(&*p),
            Self::LongArray(p) => Some(&*p),
            //Self::List(p) => {
            //    Some(&p.iter().map(|v| v.to_type_rec().unwrap() as &dyn Any).collect::<Vec<_>>() as &dyn Any)
            //},
            Self::Compound(p) => None,
            _ => None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ID {
    End, Byte, Short, Int, Long, Float, Double, ByteArray, String, List, Compound, IntArray, LongArray
}

impl ID {
    fn to_raw(&self) -> i8 {
        *self as i8
    }

    fn from_raw(id: i8) -> Self {
        match id {
            0x0 => ID::End,
            0x1 => ID::Byte,
            0x2 => ID::Short,
            0x3 => ID::Int,
            0x4 => ID::Long,
            0x5 => ID::Float,
            0x6 => ID::Double,
            0x7 => ID::ByteArray,
            0x8 => ID::String,
            0x9 => ID::List,
            0xA => ID::Compound,
            0xB => ID::IntArray,
            0xC => ID::LongArray,
            _ => panic!()
        }
    }

    fn stringify(&self) -> &'static str {
        // had a match for bytes and didn't wanna rewrite it
        match self.to_raw() {
            0x0 => "END",
            0x1 => "BYTE",
            0x2 => "SHORT",
            0x3 => "INT",
            0x4 => "LONG",
            0x5 => "FLOAT",
            0x6 => "DOUBLE",
            0x7 => "BYTE ARRAY",
            0x8 => "STRING",
            0x9 => "LIST",
            0xA => "COMPOUND",
            0xB => "INT ARRAY",
            0xC => "LONG ARRAY",
            _ => "ID ERROR"
        }
    }
}