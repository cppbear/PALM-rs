// Answer 0

#[test]
fn test_writer_flush() {
    struct TestBufMut {
        data: Vec<u8>,
    }
    
    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.data.len()
        }
        
        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }
        
        fn has_remaining_mut(&self) -> bool {
            !self.data.is_empty()
        }
        
        fn chunk_mut(&mut self) -> &mut UninitSlice {
            // Mock implementation for testing purposes.
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        // Implementing only required methods for this test
        fn put_bytes(&mut self, val: u8, cnt: usize) {
            self.data.extend(vec![val; cnt]);
        }

        fn put_u8(&mut self, n: u8) {
            self.data.push(n);
        }

        fn put_i8(&mut self, n: i8) {
            self.data.push(n as u8);
        }

        fn put_u16(&mut self, n: u16) {
            self.data.extend(&n.to_ne_bytes());
        }

        fn put_u32(&mut self, n: u32) {
            self.data.extend(&n.to_ne_bytes());
        }

        fn put_u64(&mut self, n: u64) {
            self.data.extend(&n.to_ne_bytes());
        }

        fn put_u128(&mut self, n: u128) {
            self.data.extend(&n.to_ne_bytes());
        }

        fn put_int(&mut self, n: i64, nbytes: usize) {
            let bytes = n.to_ne_bytes();
            self.data.extend_from_slice(&bytes[..nbytes]);
        }

        fn limit(self, _limit: usize) -> Limit<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }
    }

    let buf_mut = TestBufMut { data: Vec::new() };
    let mut writer = Writer { buf: buf_mut };
    
    let result = writer.flush();
    assert_eq!(result, Ok(()));
}

