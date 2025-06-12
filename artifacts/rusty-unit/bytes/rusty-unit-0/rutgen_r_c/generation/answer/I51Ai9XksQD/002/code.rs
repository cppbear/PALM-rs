// Answer 0

#[test]
fn test_chunk_with_no_remaining_in_chain() {
    struct MockBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}

        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    let buf_a = MockBuf { data: vec![1, 2, 3], position: 3 };
    let buf_b = MockBuf { data: vec![4, 5, 6], position: 0 };
    let chain = Chain { a: buf_a, b: buf_b };

    assert_eq!(chain.chunk(), &[4, 5, 6]);
}

