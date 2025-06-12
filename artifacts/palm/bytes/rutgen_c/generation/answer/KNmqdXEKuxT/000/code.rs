// Answer 0

#[test]
fn test_last_ref() {
    struct MockBuf<'a>(&'a [u8]);

    impl<'a> Buf for MockBuf<'a> {
        // Implement necessary methods for Buf trait
    }

    let a = MockBuf(&b"hello"[..]);
    let b = MockBuf(&b"world"[..]);
    let chain = Chain::new(a, b);

    assert_eq!(chain.last_ref().0, &b"world"[..]);
}

#[test]
fn test_last_ref_empty() {
    struct MockBufEmpty;

    impl Buf for MockBufEmpty {
        // Implement necessary methods returning empty slice, if required.
    }

    let a = MockBuf(&b""[..]);
    let b = MockBuf(&b"data"[..]);
    let chain = Chain::new(a, b);

    assert_eq!(chain.last_ref().0, &b"data"[..]);
}

