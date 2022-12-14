use crate::parser::raw;
use crate::parser::tag::{self, Tag};

use nom::IResult;
use nom::combinator::{self, map};
use nom::number::complete::be_u32;
use nom::multi::{count, many_till};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Payload {
    End, //Unused
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Payload>),
    Compound,
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Payload {
    pub fn end() -> Self { Self::End }
    pub fn byte(&x: &i8) -> Self { Self::Byte(x) }
    pub fn short(&x: &i16) -> Self { Self::Short(x) }
    pub fn int(&x: &i32) -> Self { Self::Int(x) }
    pub fn long(&x: &i64) -> Self { Self::Long(x) }
    pub fn float(&x: &f32) -> Self { Self::Float(x) }
    pub fn double(&x: &f64) -> Self { Self::Double(x) }
    pub fn byte_array(x: Vec<u8>) -> Self { Self::ByteArray(x) }
    pub fn string(x: &str) -> Self { Self::String(String::from(x)) }
    pub fn list(x: Vec<Payload>) -> Self { Self::List(x) }
    pub fn compound(x: Vec<u8>) -> Self { Self::ByteArray(x) }
    pub fn int_array(x: Vec<i32>) -> Self { Self::IntArray(x) }
    pub fn long_array(x: Vec<i64>) -> Self { Self::LongArray(x) }

    pub fn id(&self) -> i8 {
        use Payload::*;
        match self {
            End => 0,
            Byte(_) => 1,
            Short(_) => 2,
            Int(_) => 3,
            Long(_) => 4,
            Float(_) => 5,
            Double(_) => 6,
            ByteArray(_) => 7,
            String(_) => 8,
            List(_) => 9,
            Compound => 10,
            IntArray(_) => 11,
            LongArray(_) => 12,
        }
    }
}

pub fn payload<'a>(id: i8) -> impl Fn(&[u8]) -> IResult<&[u8], Payload> {
    match id {
        1 => byte,
        2 => short,
        3 => int,
        4 => long,
        5 => float,
        6 => double,
        7 => byte_array,
        8 => string,
        9 => list,
        10 => todo!("compound"),
        11 => int_array,
        12 => long_array,
        _ => fail,
    }
}

pub fn fail(input: &[u8]) -> IResult<&[u8], Payload> {
    combinator::fail(input)
}

pub fn byte(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::byte, into)(input)
}

pub fn short(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::short, into)(input)
}

pub fn int(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::int, into)(input)
}

pub fn long(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::long, into)(input)
}

pub fn float(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::float, into)(input)
}

pub fn double(input: &[u8]) -> IResult<&[u8], Payload> {
    map(raw::double, into)(input)
}

pub fn byte_array(input: &[u8]) -> IResult<&[u8], Payload> {
    let (rest, c) = be_u32(input)?;
    map(raw::byte_array(c as usize), into)(rest)
}

pub fn string<'a>(input: &'a [u8]) -> IResult<&'a [u8], Payload> {
    let (rest, len) = raw::ushort(input)?;
    map(raw::string(len as usize), into)(rest)
}

pub fn list<'a>(input: &'a [u8]) -> IResult<&'a [u8], Payload> {
    let (rest, id) = raw::byte(input)?;
    let (rest, c) = be_u32(rest)?;
    map(count(payload(id), c as usize), into)(rest)
}

pub fn compound<'a>(input: &'a [u8]) -> IResult<&'a [u8], Vec<Tag>> {
    map(many_till(tag::tag, tag::end), |(v,_)| v)(input)
}

pub fn int_array<'a>(input: &'a [u8]) -> IResult<&'a [u8], Payload> {
    let (rest, c) = be_u32(input)?;
    map(raw::int_array(c as usize), into)(rest)
}

pub fn long_array<'a>(input: &'a [u8]) -> IResult<&'a [u8], Payload> {
    let (rest, c) = be_u32(input)?;
    map(raw::long_array(c as usize), into)(rest)
}

pub struct TPayload<T: Into<Payload>> (T);

impl From<i8> for Payload {
    fn from(i: i8) -> Self {
        Self::Byte(i)
    }
}

impl From<i16> for Payload {
    fn from(i: i16) -> Self {
        Self::Short(i)
    }
}

impl From<i32> for Payload {
    fn from(i: i32) -> Self {
        Self::Int(i)
    }
}

impl From<i64> for Payload {
    fn from(i: i64) -> Self {
        Self::Long(i)
    }
}

impl From<f32> for Payload {
    fn from(i: f32) -> Self {
        Self::Float(i)
    }
}

impl From<f64> for Payload {
    fn from(i: f64) -> Self {
        Self::Double(i)
    }
}

impl From<&[u8]> for Payload {
    fn from(i: &[u8]) -> Self {
        Self::ByteArray(i.into())
    }
}

impl From<Vec<Payload>> for Payload {
    fn from(i: Vec<Payload>) -> Self {
        let id = i[0].id();
        i.iter().for_each(|p| if p.id() != id { panic!("Vector is not of uniform payload type") });
        Self::List(i)
    }
}

impl From<&str> for Payload {
    fn from(i: &str) -> Self {
        Self::String(i.into())
    }
}

impl From<Vec<i32>> for Payload {
    fn from(i: Vec<i32>) -> Self {
        Self::IntArray(i)
    }
}

impl From<Vec<i64>> for Payload {
    fn from(i: Vec<i64>) -> Self {
        Self::LongArray(i)
    }
}

impl From<&[i32]> for Payload {
    fn from(i: &[i32]) -> Self {
        Self::IntArray(i.to_vec())
    }
}

impl From<&[i64]> for Payload {
    fn from(i: &[i64]) -> Self {
        Self::LongArray(i.to_vec())
    }
}

#[inline]
fn into<F, T: From<F>>(a: F) -> T {
    a.into()
}

#[cfg(test)]
mod tests {
    const NUMBER_IN: [u8; 24] = [
        0x40, 0x09, 0x21, 0xff,
        0x2e, 0x48, 0xe8, 0xa7,
        0x01, 0x23, 0x45, 0x67, // 16 i8
        0x89, 0xab, 0xcd, 0xef, // 8 i16
        0x0F, 0x2D, 0x4B, 0x69, // 4 i32/f32
        0x87, 0xa5, 0xc3, 0xe1, // 2 i64/f64
        ];
    const BYTE_OUT: &[i8] = &[0x40, 0x09];
    const SHORT_OUT: &[i16] = &[0x4009, 0x21ff];
    const INT_OUT: &[i32] = &[0x400921ff, 0x2e48e8a7];
    const LONG_OUT: &[i64] = &[0x400921ff2e48e8a7, 0x0123456789abcdef];
    const FLOAT_OUT: &[f32] = &[2.1427, 4.56813719e-11];
    const DOUBLE_OUT: &[f64] = &[3.1416, 3.512700564088504e-303];

    const BYTE_ARRAY_IN: &[u8] = &[
        0x00, 0x00, 0x00, 0x03, // length
        0x01, 0x23, 0x45,       // array
        0x67, 0x89,             // rest
        ];
    const BYTE_ARRAY_OUT: &[u8] = &[0x01, 0x23, 0x45];

    static STRING_IN: &[u8] = &[
        0x00, 0x0B,             // length

        0x68, 0x65, 0x6c, 0x6c,
        0x6f, 0x20, 0x77, 0x6f,
        0x72, 0x6c, 0x64,       // string

        0x67, 0x89,             // rest
        ];
    static STRING_OUT: &str = "hello world";

    static INT_ARRAY_IN: &[u8] = &[
        0x00, 0x00, 0x00, 0x02, // array length

        0x01, 0x23, 0x45, 0x67, // 1
        0x89, 0xab, 0xcd, 0xef, // 2

        0x89, 0xab, 0xcd, 0xef, // rest
        ];
    static INT_ARRAY_OUT: &[i32] = &[0x01234567, -0x76543211];

    static LONG_ARRAY_IN: &[u8] = &[
        0x00, 0x00, 0x00, 0x02, // array length

        0x01, 0x23, 0x45, 0x67,
        0x89, 0xab, 0xcd, 0xef, // 1
        0x0F, 0x2D, 0x4B, 0x69,
        0x87, 0xa5, 0xc3, 0xe1, // 2

        0x89, 0xab, 0xcd, 0xef, // rest
        ];
    static LONG_ARRAY_OUT: &[i64] = &[0x0123456789abcdef, 0x0f2d4b6987a5c3e1];

    mod from {
        use crate::parser::payload::Payload;

        #[test]
        fn from_i8() {
            assert_eq!(Into::<Payload>::into(123i8), Payload::Byte(123));
            assert_eq!(Payload::from(123i8), Payload::Byte(123));
        }

        #[test]
        fn from_i16() {
            assert_eq!(Into::<Payload>::into(123i16), Payload::Short(123));
            assert_eq!(Payload::from(123i16), Payload::Short(123));
        }

        #[test]
        fn from_i32() {
            assert_eq!(Into::<Payload>::into(123i32), Payload::Int(123));
            assert_eq!(Payload::from(123i32), Payload::Int(123));
        }

        #[test]
        fn from_i64() {
            assert_eq!(Into::<Payload>::into(123i64), Payload::Long(123));
            assert_eq!(Payload::from(123i64), Payload::Long(123));
        }

        #[test]
        fn from_f32() {
            assert_eq!(Into::<Payload>::into(12.3f32), Payload::Float(12.3));
            assert_eq!(Payload::from(12.3f32), Payload::Float(12.3));
        }

        #[test]
        fn from_f64() {
            assert_eq!(Into::<Payload>::into(12.3f64), Payload::Double(12.3));
            assert_eq!(Payload::from(12.3f64), Payload::Double(12.3));
        }

        #[test]
        fn from_byte_slice() {
            let v = super::BYTE_ARRAY_IN.to_vec();
            assert_eq!(Into::<Payload>::into(super::BYTE_ARRAY_IN), Payload::ByteArray(v.clone()));
            assert_eq!(Payload::from(super::BYTE_ARRAY_IN), Payload::ByteArray(v.clone()));
        }

        #[test]
        fn from_string() {
            let s = "hello world";
            assert_eq!(Into::<Payload>::into(s), Payload::String(s.into()));
            assert_eq!(Payload::from(s), Payload::String(s.into()));
        }

        #[test]
        //#[ignore]
        fn from_vec() {
            let s = vec![Payload::Byte(1), Payload::Byte(2)];
            assert_eq!(Into::<Payload>::into(s.clone()), Payload::List(s.clone()));
            assert_eq!(Payload::from(s.clone()), Payload::List(s.clone()));
        }

        #[test]
        #[should_panic(expected = "Vector is not of uniform payload type")]
        fn from_nonuniform_vec() {
            let s = vec![Payload::Byte(1), Payload::Short(2)];
            assert_eq!(Into::<Payload>::into(s.clone()), Payload::List(s.clone()));
            assert_eq!(Payload::from(s.clone()), Payload::List(s.clone()));
        }

        #[test]
        fn from_int_slice() {
            let v = unsafe { std::mem::transmute::<_, Vec<i32>>(super::NUMBER_IN.to_vec()) };
            assert_eq!(Into::<Payload>::into(&v[..]), Payload::IntArray(v.clone()));
            assert_eq!(Payload::from(&v[..]), Payload::IntArray(v.clone()));
        }

        #[test]
        fn from_long_slice() {
            let v = unsafe { std::mem::transmute::<_, Vec<i64>>(super::NUMBER_IN.to_vec()) };
            assert_eq!(Into::<Payload>::into(&v[..]), Payload::LongArray(v.clone()));
            assert_eq!(Payload::from(&v[..]), Payload::LongArray(v.clone()));
        }
    }

    use crate::parser::payload::{self, Payload};

    #[test]
    fn byte() {
        let p: Payload = Payload::Byte(BYTE_OUT[0]);
        assert_eq!(payload::byte(&NUMBER_IN), Ok((&NUMBER_IN[1..], p)));
    }

    #[test]
    fn short() {
        let p: Payload = Payload::Short(SHORT_OUT[0]);
        assert_eq!(payload::short(&NUMBER_IN), Ok((&NUMBER_IN[2..], p)));
    }

    #[test]
    fn int() {
        let p: Payload = Payload::Int(INT_OUT[0]);
        assert_eq!(payload::int(&NUMBER_IN), Ok((&NUMBER_IN[4..], p)));
    }

    #[test]
    fn long() {
        let p: Payload = Payload::Long(LONG_OUT[0]);
        assert_eq!(payload::long(&NUMBER_IN), Ok((&NUMBER_IN[8..], p)));
    }

    #[test]
    fn float() {
        let p: Payload = Payload::Float(FLOAT_OUT[0]);
        assert_eq!(payload::float(&NUMBER_IN), Ok((&NUMBER_IN[4..], p)));
    }

    #[test]
    fn double() {
        let p: Payload = Payload::Double(DOUBLE_OUT[0]);
        assert_eq!(payload::double(&NUMBER_IN), Ok((&NUMBER_IN[8..], p)));
    }

    #[test]
    fn byte_array() {
        let p = Payload::ByteArray(BYTE_ARRAY_OUT.to_vec());
        assert_eq!(payload::byte_array(BYTE_ARRAY_IN), Ok((&BYTE_ARRAY_IN[7..], p)));
    }

    #[test]
    fn string() {
        let p = Payload::String(STRING_OUT.to_string());
        assert_eq!(payload::string(STRING_IN), Ok((&STRING_IN[13..], p)));
    }

    #[test]
    fn list() {
        let mut lin: Vec<u8> = Vec::with_capacity(5+20*2);
        lin.resize(5 + NUMBER_IN.len(), 0);
        lin.splice(1..5, [0,0,0,2]);

        lin[0] = 1; // Byte
        lin.splice(5.., NUMBER_IN);
        let p = Payload::List(BYTE_OUT.into_iter().map(Payload::byte).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[2..], p)));

        lin[0] = 2; // Short
        let p = Payload::List(SHORT_OUT.into_iter().map(Payload::short).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[4..], p)));

        lin[0] = 3; // Int
        let p = Payload::List(INT_OUT.into_iter().map(Payload::int).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[8..], p)));

        lin[0] = 4; // Long
        let p = Payload::List(LONG_OUT.into_iter().map(Payload::long).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[16..], p)));

        lin[0] = 5; // Float
        let p = Payload::List(FLOAT_OUT.into_iter().map(Payload::float).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[8..], p)));

        lin[0] = 6; // Double
        let p = Payload::List(DOUBLE_OUT.into_iter().map(Payload::double).collect());
        assert_eq!(payload::list(&lin), Ok((&NUMBER_IN[16..], p)));

        lin[0] = 7; // Byte array
        lin.resize(5 + 7 * 2, 0);
        lin.splice(5.., BYTE_ARRAY_IN[..7].repeat(2));
        let p = Payload::List(vec![
            Payload::ByteArray(BYTE_ARRAY_OUT.to_vec()),
            Payload::ByteArray(BYTE_ARRAY_OUT.to_vec())
        ]);
        assert_eq!(payload::list(&lin), Ok((&[] as &[u8], p)));

        lin[0] = 8; // String
        lin.resize(5 + 13 * 2, 0);
        lin.splice(5.., STRING_IN[..13].repeat(2));
        let p = Payload::List(vec![
            Payload::String(String::from(STRING_OUT)),
            Payload::String(String::from(STRING_OUT))
        ]);
        assert_eq!(payload::list(&lin), Ok((&[] as &[u8], p)));
        // lin[0] = 9; // List
        // lin[0] = 10; // Compound

        lin[0] = 11; // Int array
        lin.resize(5 + 12 * 2, 0);
        lin.splice(5.., INT_ARRAY_IN[..12].repeat(2));
        let p = Payload::List(vec![
            Payload::IntArray(INT_ARRAY_OUT.to_vec()),
            Payload::IntArray(INT_ARRAY_OUT.to_vec())
        ]);
        assert_eq!(payload::list(&lin), Ok((&[] as &[u8], p)));

        lin[0] = 12; // Long array
        lin.resize(5 + 20 * 2, 0);
        lin.splice(5.., LONG_ARRAY_IN[..20].repeat(2));
        let p = Payload::List(vec![
            Payload::LongArray(LONG_ARRAY_OUT.to_vec()),
            Payload::LongArray(LONG_ARRAY_OUT.to_vec())
        ]);
        assert_eq!(payload::list(&lin), Ok((&[] as &[u8], p)));

    }

    #[test]
    #[ignore]
    fn compound() {
        todo!("Compound payload test");
    }

    #[test]
    fn int_array() {
        let p = Payload::IntArray(INT_ARRAY_OUT.to_vec());
        assert_eq!(payload::int_array(INT_ARRAY_IN), Ok((&INT_ARRAY_IN[12..], p)));
    }

    #[test]
    fn long_array() {
        let p = Payload::LongArray(LONG_ARRAY_OUT.to_vec());
        assert_eq!(payload::long_array(LONG_ARRAY_IN), Ok((&LONG_ARRAY_IN[20..], p)));
    }

    #[test]
    fn payload() {
        // Modified int array input, not worth testing every type
        let LIST: &[u8] = &[
            0x03, //id
            0x00, 0x00, 0x00, 0x02, // array length

            0x01, 0x23, 0x45, 0x67, // 1
            0x89, 0xab, 0xcd, 0xef, // 2

            0x89, 0xab, 0xcd, 0xef, // rest
            ];

        assert_eq!(payload::byte(&NUMBER_IN), payload::payload(1)(&NUMBER_IN));
        assert_eq!(payload::short(&NUMBER_IN), payload::payload(2)(&NUMBER_IN));
        assert_eq!(payload::int(&NUMBER_IN), payload::payload(3)(&NUMBER_IN));
        assert_eq!(payload::long(&NUMBER_IN), payload::payload(4)(&NUMBER_IN));
        assert_eq!(payload::float(&NUMBER_IN), payload::payload(5)(&NUMBER_IN));
        assert_eq!(payload::double(&NUMBER_IN), payload::payload(6)(&NUMBER_IN));
        assert_eq!(payload::byte_array(BYTE_ARRAY_IN), payload::payload(7)(BYTE_ARRAY_IN));
        assert_eq!(payload::string(STRING_IN), payload::payload(8)(STRING_IN));
        assert_eq!(payload::list(LIST), payload::payload(9)(LIST));
        // todo!("Payload 10 (compound) test");
        assert_eq!(payload::int_array(INT_ARRAY_IN), payload::payload(11)(INT_ARRAY_IN));
        assert_eq!(payload::long_array(LONG_ARRAY_IN), payload::payload(12)(LONG_ARRAY_IN));
    }
}