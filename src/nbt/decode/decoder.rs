use bytes::buf::Buf;

pub struct Decoder<'a>(&'a mut dyn Buf);

impl Decoder<'_> {
    pub fn new(buf:&mut dyn Buf) -> Decoder {
        Decoder(buf)
    }
}

impl Buf for Decoder<'_> {
    fn remaining(&self) -> usize {
        self.0.remaining()
    }

    fn bytes(&self) -> &[u8] {
        self.0.bytes()
    }

    fn advance(&mut self, cnt: usize) {
        self.0.advance(cnt)
    }
}

impl Decoder<'_> {
    pub fn read_id(&mut self) -> u8 {
        self.get_u8()
    }

    pub fn read_name(&mut self) -> String {
        let len = self.get_u16_be();
        let mut name = vec![0u8; usize::try_from(len).unwrap()];
        self.copy_to_slice(&mut name);
        String::from_utf8(name).unwrap()
    }
}