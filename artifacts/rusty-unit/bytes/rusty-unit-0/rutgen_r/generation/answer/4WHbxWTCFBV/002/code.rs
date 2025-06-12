// Answer 0

#[test]
fn test_put_bytes_insufficient_capacity() {
    use std::ptr;

    struct BytesMut {
        data: Vec<u8>,
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn new() -> Self {
            Self {
                data: Vec::new(),
                cap: 0,
                len: 0,
            }
        }

        fn reserve(&mut self, cnt: usize) {
            if self.cap < self.len + cnt {
                self.cap = self.len + cnt;
                self.data.resize(self.cap, 0);
            }
        }

        fn spare_capacity_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.len..self.cap]
        }

        fn advance_mut(&mut self, cnt: usize) {
            self.len += cnt;
        }

        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.reserve(cnt);
            unsafe {
                let dst = self.spare_capacity_mut();
                debug_assert!(dst.len() >= cnt);

                ptr::write_bytes(dst.as_mut_ptr(), val, cnt);

                self.advance_mut(cnt);
            }
        }
    }

    let mut bytes = BytesMut::new();
    // Reserve less than the amount we want to write to trigger panic
    bytes.reserve(2); // Let's reserve an initial capacity of 2

    // This call will trigger the debug assertion because we are trying to write 4 bytes
    let result = std::panic::catch_unwind(|| {
        bytes.put_bytes(1, 4);
    });

    assert!(result.is_err());
}

