// Answer 0

#[test]
fn test_freeze_with_kind_vec() {
    use bytes::{BytesMut, Bytes};

    struct TestVec {
        data: Vec<u8>,
    }

    impl TestVec {
        fn new(len: usize) -> Self {
            Self {
                data: vec![0; len],
            }
        }

        fn put(&mut self, bytes: &[u8]) {
            self.data.extend_from_slice(bytes);
        }
        
        fn freeze(self) -> Bytes {
            let mut b = BytesMut::with_capacity(self.data.len());
            b.put(&self.data);
            b.freeze()
        }
    }

    let test_vec = TestVec::new(64);
    let mut b = test_vec;
    
    b.put(&b"hello world"[..]);
    let frozen_bytes = b.freeze();

    assert_eq!(&frozen_bytes[..], b"hello world");
}

#[test]
fn test_freeze_empty_vec() {
    use bytes::{BytesMut, Bytes};

    let mut b = BytesMut::with_capacity(0);
    let frozen_bytes = b.freeze();

    assert_eq!(&frozen_bytes[..], b"");
}

