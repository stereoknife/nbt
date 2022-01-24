use std::io::Read;
use bytes::buf::Buf;
use compress::zlib;

#[derive(Debug)]
pub enum Error {
	CompressionFormatError(u8)
}

pub fn parse_locations(buf: &mut dyn Buf) -> Vec<(usize,usize)> {
	let mut out = Vec::new();
	while buf.has_remaining() {
		let mut loc = [0u8; 8];
		buf.copy_to_slice(&mut loc[5..]);
		let pos = usize::from_be_bytes(loc);
		let siz = usize::try_from(buf.get_u8()).unwrap();
		out.push((pos, siz));
	}
	out
}

pub fn parse_timestamps(buf: &mut dyn Buf) -> Vec<u32> {
	let mut out = Vec::new();
	while buf.has_remaining() {
		out.push(buf.get_u32_be());
	}
	out
}

pub fn extract_chunk(buf: &mut dyn Buf) -> Result<Vec<u8>, Error> {
	let size = usize::try_from(buf.get_u32_be()).unwrap();
	println!("chunk data size {} bytes", size);
	let comp = buf.get_u8();
	println!("compression method {}", comp);
	if comp != 2 { return Err(Error::CompressionFormatError(comp)) }
	let compressed = buf.take(size);
	let mut decompressed = Vec::new();
	zlib::Decoder::new(compressed.reader()).read_to_end(&mut decompressed).unwrap();
	Ok(decompressed)
}