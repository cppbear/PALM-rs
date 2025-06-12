// Answer 0

#[test]
fn test_get_mut() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new() -> Self {
            MockBufMut { data: vec![1, 2, 3] }
        }
    }

    let mut buf = Limit {
        inner: MockBufMut::new(),
        limit: 3,
    };

    let inner_ref: &mut MockBufMut = buf.get_mut();
    inner_ref.data.push(4);
    assert_eq!(inner_ref.data.len(), 4);
    assert_eq!(inner_ref.data[3], 4);
}

#[test]
fn test_get_mut_empty() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new() -> Self {
            MockBufMut { data: Vec::new() }
        }
    }

    let mut buf = Limit {
        inner: MockBufMut::new(),
        limit: 0,
    };

    let inner_ref: &mut MockBufMut = buf.get_mut();
    inner_ref.data.push(1);
    assert_eq!(inner_ref.data.len(), 1);
    assert_eq!(inner_ref.data[0], 1);
}

#[test]
fn test_get_mut_large_data() {
    struct MockBufMut {
        data: Vec<u8>,
    }

    impl MockBufMut {
        fn new() -> Self {
            MockBufMut { data: vec![0; 1024] }
        }
    }

    let mut buf = Limit {
        inner: MockBufMut::new(),
        limit: 1024,
    };

    let inner_ref: &mut MockBufMut = buf.get_mut();
    inner_ref.data[0] = 255;
    assert_eq!(inner_ref.data[0], 255);
}

