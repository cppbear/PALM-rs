// Answer 0

#[test]
#[should_panic(expected = "not enough remaining capacity")]
fn test_put_bytes_panics_when_not_enough_capacity() {
    struct MockBuf {
        data: &'static mut [u8],
        size: usize,
        position: usize,
    }

    impl MockBuf {
        fn new(data: &'static mut [u8]) -> Self {
            Self { data, size: data.len(), position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.size - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }
        
        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.position..]
        }

        fn put_bytes(&mut self, val: u8, mut cnt: usize) {
            if self.remaining_mut() < cnt {
                panic!("not enough remaining capacity");
            }

            while cnt > 0 {
                let dst = self.chunk_mut();
                let dst_len = usize::min(dst.len(), cnt);
                unsafe { core::ptr::write_bytes(dst.as_mut_ptr(), val, dst_len) };
                unsafe { self.advance_mut(dst_len) };
                cnt -= dst_len;
            }
        }
    }

    let mut buffer = [0; 3]; // Total capacity is 3 bytes
    let mut buf = MockBuf::new(&mut buffer);
    
    // This will trigger a panic since remaining capacity is 3 and we request to put 5 bytes
    buf.put_bytes(b'a', 5);
}

#[test]
fn test_put_bytes_fills_buffer_correctly() {
    struct MockBuf {
        data: &'static mut [u8],
        size: usize,
        position: usize,
    }

    impl MockBuf {
        fn new(data: &'static mut [u8]) -> Self {
            Self { data, size: data.len(), position: 0 }
        }

        fn remaining_mut(&self) -> usize {
            self.size - self.position
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn chunk_mut(&mut self) -> &mut [u8] {
            &mut self.data[self.position..]
        }

        fn put_bytes(&mut self, val: u8, mut cnt: usize) {
            if self.remaining_mut() < cnt {
                panic!("not enough remaining capacity");
            }

            while cnt > 0 {
                let dst = self.chunk_mut();
                let dst_len = usize::min(dst.len(), cnt);
                unsafe { core::ptr::write_bytes(dst.as_mut_ptr(), val, dst_len) };
                unsafe { self.advance_mut(dst_len) };
                cnt -= dst_len;
            }
        }
    }

    let mut buffer = [0; 6]; // Total capacity is 6 bytes
    {
        let mut buf = MockBuf::new(&mut buffer);
        buf.put_bytes(b'a', 4);

        assert_eq!(2, buf.remaining_mut());
    }

    assert_eq!(b"aaaa\0\0", &buffer);
}

