// Answer 0

#[test]
fn test_truncate_valid_case() {
    struct Vtable;
    const PROMOTABLE_ODD_VTABLE: Vtable = Vtable;
    
    struct Bytes {
        len: usize,
        vtable: *const Vtable,
        data: Vec<u8>,
    }
    
    impl Bytes {
        fn from(slice: &[u8]) -> Self {
            Bytes {
                len: slice.len(),
                vtable: &PROMOTABLE_ODD_VTABLE,
                data: slice.to_vec(),
            }
        }
        
        fn truncate(&mut self, len: usize) {
            if len < self.len {
                if self.vtable as *const Vtable == &PROMOTABLE_EVEN_VTABLE
                    || self.vtable as *const Vtable == &PROMOTABLE_ODD_VTABLE {
                    drop(self.split_off(len));
                } else {
                    self.len = len;
                }
            }
        }
        
        fn split_off(&mut self, at: usize) -> Vec<u8> {
            let split_data = self.data.split_off(at);
            self.len = at;
            split_data
        }
    }

    let mut buf = Bytes::from(&b"hello world"[..]);
    buf.truncate(5);
    assert_eq!(buf.data, b"hello"[..]);
    assert_eq!(buf.len, 5);
}

#[test]
#[should_panic]
fn test_truncate_panic_case() {
    struct Vtable;
    const PROMOTABLE_EVEN_VTABLE: Vtable = Vtable;
    const PROMOTABLE_ODD_VTABLE: Vtable = Vtable;

    struct Bytes {
        len: usize,
        vtable: *const Vtable,
        data: Vec<u8>,
    }

    impl Bytes {
        fn from(slice: &[u8]) -> Self {
            Bytes {
                len: slice.len(),
                vtable: &PROMOTABLE_EVEN_VTABLE,
                data: slice.to_vec(),
            }
        }

        fn truncate(&mut self, len: usize) {
            if len < self.len {
                if self.vtable as *const Vtable == &PROMOTABLE_EVEN_VTABLE
                    || self.vtable as *const Vtable == &PROMOTABLE_ODD_VTABLE {
                    drop(self.split_off(len));
                } else {
                    self.len = len;
                }
            }
        }

        fn split_off(&mut self, at: usize) -> Vec<u8> {
            let split_data = self.data.split_off(at);
            self.len = at;
            split_data
        }
    }

    let mut buf = Bytes::from(&b"hello"[..]);
    buf.truncate(5); // This will cause panic since len is not less than self.len
}

