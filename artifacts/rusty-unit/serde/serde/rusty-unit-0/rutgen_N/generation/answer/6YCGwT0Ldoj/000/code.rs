// Answer 0

#[derive(Debug)]
struct Buf<'a> {
    bytes: &'a mut [u8],
    offset: usize,
}

impl<'a> Buf<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Buf { bytes, offset: 0 }
    }
}

#[test]
fn test_buf_new() {
    let mut data = [0u8; 10];
    let buf = Buf::new(&mut data);
    assert_eq!(buf.bytes.as_ptr(), data.as_mut_ptr());
    assert_eq!(buf.offset, 0);
}

#[test]
fn test_buf_new_with_non_zero_offset() {
    let mut data = [1u8; 10];
    let mut buf = Buf::new(&mut data);
    buf.offset = 5; // simulating offset change
    assert_eq!(buf.bytes.as_ptr(), data.as_mut_ptr());
    assert_eq!(buf.offset, 5);
}

#[test]
#[should_panic]
fn test_buf_new_invalid_reference() {
    let data = [0u8; 10];
    let _buf = Buf::new(&mut data); // This should panic because data is not mutable
}

