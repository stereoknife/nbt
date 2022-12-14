pub mod raw;
pub mod payload;
pub mod tag;

mod tests {
    static BYTE_TAG: &[u8] = &[
        0x01,                   //id byte
        0x00, 0x04,             //name length
        0x74, 0x65, 0x73, 0x74, //name: test
        0x01,                   //value: 1
        ];

    static SHORT_TAG: &[u8] = &[
        0x02,                   //id short
        0x00, 0x04,             //name length
        0x74, 0x65, 0x73, 0x74, //name: test
        0x01, 0x23,             //value: 0x0123
        ];

    static INT_TAG: &[u8] = &[
        0x03,                   //id int
        0x00, 0x04,             //name length
        0x74, 0x65, 0x73, 0x74, //name: test
        0x01, 0x23, 0x45, 0x67, //value: 0x01234567
        ];

    static LONG_TAG: &[u8] = &[
        0x03,                   //id long
        0x00, 0x04,             //name length
        0x74, 0x65, 0x73, 0x74, //name: test
        0x01, 0x23, 0x45, 0x67,
        0x89, 0xAB, 0xCD, 0xEF, //value: 0x0123456789ABCDEF
        ];
}
