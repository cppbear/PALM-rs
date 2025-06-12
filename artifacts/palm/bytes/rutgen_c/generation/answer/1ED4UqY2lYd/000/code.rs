// Answer 0

#[test]
fn test_reader_get_mut() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.position < self.data.len()
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let len = cmp::min(dst.len(), self.remaining());
            dst[..len].copy_from_slice(&self.data[self.position..self.position + len]);
            self.position += len;
        }

        fn get_u8(&mut self) -> u8 {
            let byte = self.data[self.position];
            self.position += 1;
            byte
        }

        fn get_i8(&mut self) -> i8 {
            self.get_u8() as i8
        }

        fn get_u16(&mut self) -> u16 {
            // simplified for testing
            (self.get_u8() as u16) << 8 | self.get_u8() as u16
        }

        fn get_u32(&mut self) -> u32 {
            (self.get_u16() as u32) << 16 | self.get_u16() as u32
        }

        fn get_u64(&mut self) -> u64 {
            (self.get_u32() as u64) << 32 | self.get_u32() as u64
        }

        fn get_u128(&mut self) -> u128 {
            (self.get_u64() as u128) << 64 | self.get_u64() as u128
        }

        // Other methods are omitted for brevity
    }

    let mut buf = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        position: 0,
    };
    let mut reader = Reader { buf };

    let inner_buf = reader.get_mut();
    inner_buf.advance(2);

    assert_eq!(inner_buf.get_u8(), 3);
    assert_eq!(inner_buf.remaining(), 3);
}

