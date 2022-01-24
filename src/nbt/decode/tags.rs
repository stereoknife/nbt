use super::super::types::Tag;
use bytes::buf::Buf;
use super::decoder::Decoder;

impl Decoder<'_> {
    pub fn read_byte (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_byte_payload();
        Tag::new(name, val)
    }
    
    pub fn read_short (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_short_payload();
        Tag::new(name, val)
    }
    
    pub fn read_int (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_int_payload();
        Tag::new(name, val)
    }
    
    pub fn read_long (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_long_payload();
        Tag::new(name, val)
    }
    
    pub fn read_float (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_float_payload();
        Tag::new(name, val)
    }
    
    pub fn read_double (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_double_payload();
        Tag::new(name, val)
    }
    
    pub fn read_byte_array (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_byte_array_payload();
        Tag::new(name, val)
    }
    
    pub fn read_string (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_string_payload();
        Tag::new(name, val)
    }
    
    pub fn read_list (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_list_payload();
        Tag::new(name, val)
    }
    
    pub fn read_compound (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_compound_payload();
        Tag::new(name, val)
    }
    
    pub fn read_int_array (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_int_array_payload();
        Tag::new(name, val)
    }
    
    pub fn read_long_array (&mut self) -> Tag {
        let name = self.read_name();
        let val = self.read_long_array_payload();
        Tag::new(name, val)
    }

    pub fn read_tag(&mut self) -> Tag {
        let id = self.get_i8();
        if id == 0 { return Tag::End }
        let name = self.read_name();
        let val = self.read_payload(id);
        Tag::new(name, val)
    }
}