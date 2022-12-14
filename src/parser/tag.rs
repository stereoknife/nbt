use crate::parser::payload::{self, Payload};
use crate::parser::raw;

use nom::IResult;
use nom::combinator::{map, verify, value};
use nom::bytes::complete as bytes;

pub struct Tag {
    pub name: String,
    pub payload: Payload
}

pub fn end(input: &[u8]) -> IResult<&[u8], ()> {
    value((), bytes::tag([0u8]))(input)
}

pub fn tag(input: &[u8]) -> IResult<&[u8], Tag> {
    let (rest, id) = verify(raw::byte, |&i| i > 0 && i <= 12)(input)?;
    let (rest, nlen) = raw::ushort(rest)?;
    let (rest, name) = map(raw::string(nlen as usize), String::from)(rest)?;
    let (rest, payload) = payload::payload(id)(rest)?;
    Ok((rest, Tag { name, payload }))
}