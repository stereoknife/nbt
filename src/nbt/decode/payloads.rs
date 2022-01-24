use super::super::types::{Payload, Tag};
use super::decoder::Decoder;
use bytes::buf::Buf;
use std::collections::HashMap;

impl Decoder<'_> {
    pub fn read_byte_payload (&mut self) -> Payload {
        let payload = self.get_i8();
        Payload::Byte(payload)
    }
    
    pub fn read_short_payload (&mut self) -> Payload {
        let payload = self.get_i16_be();
        Payload::Short(payload)
    }
    
    pub fn read_int_payload (&mut self) -> Payload {
        let payload = self.get_i32_be();
        Payload::Int(payload)
    }
    
    pub fn read_long_payload (&mut self) -> Payload {
        let payload = self.get_i64_be();
        Payload::Long(payload)
    }
    
    pub fn read_float_payload (&mut self) -> Payload {
        let payload = self.get_f32_be();
        Payload::Float(payload)
    }
    
    pub fn read_double_payload (&mut self) -> Payload {
        let payload = self.get_f64_be();
        Payload::Double(payload)
    }
    
    pub fn read_byte_array_payload (&mut self) -> Payload {
        let len = self.get_i32_be();
        let mut payload = Vec::new();
        for _ in 0..len {
            payload.push(self.get_u8());
        }
        Payload::ByteArray(payload)
    }
    
    pub fn read_string_payload (&mut self) -> Payload {
        let len = self.get_u16_be();
        let mut bstr = vec![0u8; usize::try_from(len).unwrap()];
        self.copy_to_slice(&mut bstr);
        let payload = String::from_utf8(bstr).unwrap();
        Payload::String(payload)
    }
    
    pub fn read_list_payload (&mut self) -> Payload {
        let id = self.get_i8();
        let len = self.get_i32_be();
        let mut payload = Vec::new();
        for _ in 0..len {
            let next = self.read_payload(id);
            payload.push(next);
        }
        Payload::List(payload)
    }


    pub fn read_compound_payload(&mut self) -> Payload {
        let mut payload = HashMap::new();
        loop {
            let next = self.read_tag();
            if let Tag::End = next { break; }
            let name = next.name().unwrap();
            payload.insert(name.clone(), next);
        }
        Payload::Compound(payload)
    }
    
    pub fn read_int_array_payload (&mut self) -> Payload {
        let len = self.get_i32_be();
        let mut payload = Vec::new();
        for _ in 0..len {
            payload.push(self.get_i32_be());
        }
        Payload::IntArray(payload)
    }
    
    pub fn read_long_array_payload (&mut self) -> Payload {
        let len = self.get_i32_be();
        let mut payload = Vec::new();
        for _ in 0..len {
            payload.push(self.get_i64_be());
        }
        Payload::LongArray(payload)
    }

    pub fn read_payload (&mut self, id: i8) -> Payload {
        match id {
            0x1 => self.read_byte_payload(),
            0x2 => self.read_short_payload(),
            0x3 => self.read_int_payload(),
            0x4 => self.read_long_payload(),
            0x5 => self.read_float_payload(),
            0x6 => self.read_double_payload(),
            0x7 => self.read_byte_array_payload(),
            0x8 => self.read_string_payload(),
            0x9 => self.read_list_payload(),
            0xA => self.read_compound_payload(),
            0xB => self.read_int_array_payload(),
            0xC => self.read_long_array_payload(),
            a => panic!("Invalid NBT id: {}", a)
        }
    }
}