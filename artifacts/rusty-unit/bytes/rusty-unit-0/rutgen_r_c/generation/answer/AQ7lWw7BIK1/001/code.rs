// Answer 0

#[test]
fn test_into_inner_basic() {
    struct MyBuf {
        data: Vec<u8>,
    }
    impl MyBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    let buf = MyBuf { data: vec![b'a', b'b', b'c'] };
    let iter = IntoIter::new(buf);
    let inner = iter.into_inner();
    assert_eq!(inner.remaining(), 3);
}

#[test]
fn test_into_inner_empty() {
    struct MyBuf {
        data: Vec<u8>,
    }
    impl MyBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    let buf = MyBuf { data: Vec::new() };
    let iter = IntoIter::new(buf);
    let inner = iter.into_inner();
    assert_eq!(inner.remaining(), 0);
}

#[test]
fn test_into_inner_non_empty() {
    struct MyBuf {
        data: Vec<u8>,
    }
    impl MyBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }
    }

    let buf = MyBuf { data: vec![1, 2, 3, 4, 5] };
    let iter = IntoIter::new(buf);
    let inner = iter.into_inner();
    assert_eq!(inner.remaining(), 5);
}

#[should_panic]
fn test_into_inner_panic() {
    struct MyBuf {
        data: Vec<u8>,
    }
    impl MyBuf {
        fn remaining(&self) -> usize {
            if self.data.is_empty() {
                panic!("Buffer is empty!")
            }
            self.data.len()
        }
    }

    let buf = MyBuf { data: Vec::new() };
    let iter = IntoIter::new(buf);
    let _inner = iter.into_inner();
}

