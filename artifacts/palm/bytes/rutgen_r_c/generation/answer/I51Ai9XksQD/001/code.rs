// Answer 0

#[test]
fn test_chunk_with_a_having_remaining() {
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
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, dst: &mut [u8]) {
            let len = self.remaining().min(dst.len());
            dst[..len].copy_from_slice(&self.chunk()[..len]);
        }

        fn get_u8(&mut self) -> u8 {
            let val = self.chunk()[0];
            self.advance(1);
            val
        }

        // Other required methods can be left unimplemented for this test
        fn get_i8(&mut self) -> i8 { unimplemented!() }
        fn get_u16(&mut self) -> u16 { unimplemented!() }
        fn get_u32(&mut self) -> u32 { unimplemented!() }
        fn get_u64(&mut self) -> u64 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
        fn try_copy_to_slice(&mut self, dst: &mut [u8]) -> Result<(), TryGetError> { unimplemented!() }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { unimplemented!() }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { unimplemented!() }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { unimplemented!() }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { unimplemented!() }
        fn get_i16(&mut self) -> i16 { unimplemented!() }
        fn get_i32(&mut self) -> i32 { unimplemented!() }
        fn get_i64(&mut self) -> i64 { unimplemented!() }
        fn get_u128(&mut self) -> u128 { unimplemented!() }
        fn get_i128(&mut self) -> i128 { unimplemented!() }
        fn get_uint(&mut self, nbytes: usize) -> u64 { unimplemented!() }
        fn get_int(&mut self, nbytes: usize) -> i64 { unimplemented!() }
        fn get_f32(&mut self) -> f32 { unimplemented!() }
        fn get_f64(&mut self) -> f64 { unimplemented!() }
    }

    let buf_a = TestBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let buf_b = TestBuf { data: vec![6, 7, 8], position: 0 };
    let chain_buf = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain_buf.chunk(), &[1, 2, 3, 4, 5]);
}

