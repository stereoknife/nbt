use std::str;
use nom::IResult;
use nom::number::complete::*;
use nom::bytes::complete::take;
use nom::multi::count;
use nom::combinator::map;
//use crate::parser::payload::{Payload, payload};

pub fn byte(input: &[u8]) -> IResult<&[u8], i8> {
    i8(input)
}

pub fn ubyte(input: &[u8]) -> IResult<&[u8], u8> {
    u8(input)
}

pub fn short(input: &[u8]) -> IResult<&[u8], i16> {
    be_i16(input)
}

pub fn ushort(input: &[u8]) -> IResult<&[u8], u16> {
    be_u16(input)
}

pub fn int(input: &[u8]) -> IResult<&[u8], i32> {
    be_i32(input)
}

pub fn long(input: &[u8]) -> IResult<&[u8], i64> {
    be_i64(input)
}

pub fn float(input: &[u8]) -> IResult<&[u8], f32> {
    be_f32(input)
}

pub fn double(input: &[u8]) -> IResult<&[u8], f64> {
    be_f64(input)
}

pub fn byte_array<'a>(len: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    take(len)
}

pub fn int_array<'a>(len: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<i32>> {
    count(int, len)
}

pub fn long_array<'a>(len: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<i64>> {
    count(long, len)
}

pub fn string<'a>(len: usize) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], &'a str> {
    map(take(len), unsafe { |s| str::from_utf8_unchecked(s) })
}

#[cfg(test)]
mod tests {
    use super::*;

    const s: [u8; 24] = [0x40, 0x09, 0x21, 0xff,
                         0x2e, 0x48, 0xe8, 0xa7,

                         0x9a, 0x99, 0x99, 0x99,
                         0x99, 0x99, 0x1b, 0x40,

                         0x8f, 0xc2, 0xf5, 0x28,
                         0x5c, 0x0f, 0x3e, 0x40];

    #[test]
    fn byte_test() {
        assert_eq!(byte(&s[3..]), Ok((&s[4..], -0x01)))
    }

    #[test]
    fn ubyte_test() {
        assert_eq!(ubyte(&s[3..]), Ok((&s[4..], 0xff)))
    }

    #[test]
    fn short_test() {
        assert_eq!(short(&s[..]), Ok((&s[2..], 0x4009)))
    }

    #[test]
    fn int_test() {
        assert_eq!(int(&s[..]), Ok((&s[4..], 0x400921ff)))
    }

    #[test]
    fn long_test() {
        assert_eq!(long(&s[..]), Ok((&s[8..], 0x400921ff2e48e8a7)))
    }

    #[test]
    fn float_test() {
        assert_eq!(float(&s[..]), Ok((&s[4..], 2.1427)));
    }

    #[test]
    fn double_test() {
        assert_eq!(double(&s[..]), Ok((&s[8..], 3.1416)));
    }

    #[test]
    fn byte_array_test() {
        assert_eq!(byte_array(8)(&s[..]), Ok((&s[8..], &s[..8])));
    }

    #[test]
    fn int_array_test() {
        let t = vec![0x400921ff, 0x2e48e8a7, -0x65666667, -0x6666E4C0];
        assert_eq!(int_array(4)(&s[..]), Ok((&s[16..], t)));
    }

    #[test]
    fn long_array_test() {
        let t = vec![0x400921ff2e48e8a7, -0x656666666666E4C0];
        assert_eq!(long_array(2)(&s[..]), Ok((&s[16..], t)));
    }

    #[test]
    fn string_test() {
        let t = [0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0xff];
        assert_eq!(string(11)(&t[..]), Ok((&t[11..], "hello world")));
    }
}