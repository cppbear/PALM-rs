// Answer 0

#[derive(Debug)]
struct Writer<B> {
    buf: B,
}

impl<B> Writer<B> {
    pub fn into_inner(self) -> B {
        self.buf
    }
}

#[test]
fn test_into_inner_with_vector() {
    let mut buf = Writer { buf: vec![] };
    let mut src = &b"hello world"[..];
    
    std::io::copy(&mut src, &mut buf.buf).unwrap();

    let buf_inner = buf.into_inner();
    assert_eq!(buf_inner, b"hello world"[..].to_vec());
}

#[test]
fn test_into_inner_with_empty_vector() {
    let buf = Writer { buf: vec![] };
    let buf_inner = buf.into_inner();
    assert_eq!(buf_inner, &b""[..].to_vec());
}

#[test]
fn test_into_inner_with_byte_slice() {
    let buf = Writer { buf: b"test"[..].to_vec() };
    let buf_inner = buf.into_inner();
    assert_eq!(buf_inner, b"test"[..].to_vec());
}

