// Answer 0

#[test]
fn test_chain_advance_a_remains() {
    struct TestBufA {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufA {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct TestBufB {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufB {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf_a = TestBufA { data: vec![1, 2, 3, 4], position: 0 };
    let mut buf_b = TestBufB { data: vec![5, 6, 7, 8], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    chain_buf.advance(2);

    assert_eq!(chain_buf.a.remaining(), 2);
    assert_eq!(chain_buf.b.remaining(), 4);
}

#[test]
fn test_chain_advance_b_only() {
    struct TestBufA {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufA {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct TestBufB {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufB {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf_a = TestBufA { data: vec![1, 2, 3, 4], position: 4 };
    let mut buf_b = TestBufB { data: vec![5, 6, 7, 8], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    chain_buf.advance(3);

    assert_eq!(chain_buf.a.remaining(), 0);
    assert_eq!(chain_buf.b.remaining(), 1);
}

#[test]
fn test_chain_advance_b_empty() {
    struct TestBufA {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufA {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    struct TestBufB {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBufB {
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

        fn copy_to_slice(&mut self, _: &mut [u8]) {}
        fn get_u8(&mut self) -> u8 { 0 }
        fn get_i8(&mut self) -> i8 { 0 }
        fn get_u16(&mut self) -> u16 { 0 }
        fn get_u16_le(&mut self) -> u16 { 0 }
        fn get_u16_ne(&mut self) -> u16 { 0 }
        fn get_i16(&mut self) -> i16 { 0 }
        fn get_i16_le(&mut self) -> i16 { 0 }
        fn get_i16_ne(&mut self) -> i16 { 0 }
        fn get_u32(&mut self) -> u32 { 0 }
        fn get_u32_le(&mut self) -> u32 { 0 }
        fn get_u32_ne(&mut self) -> u32 { 0 }
        fn get_i32(&mut self) -> i32 { 0 }
        fn get_i32_le(&mut self) -> i32 { 0 }
        fn get_i32_ne(&mut self) -> i32 { 0 }
        fn get_u64(&mut self) -> u64 { 0 }
        fn get_u64_le(&mut self) -> u64 { 0 }
        fn get_u64_ne(&mut self) -> u64 { 0 }
        fn get_i64(&mut self) -> i64 { 0 }
        fn get_i64_le(&mut self) -> i64 { 0 }
        fn get_i64_ne(&mut self) -> i64 { 0 }
        fn get_u128(&mut self) -> u128 { 0 }
        fn get_u128_le(&mut self) -> u128 { 0 }
        fn get_u128_ne(&mut self) -> u128 { 0 }
        fn get_i128(&mut self) -> i128 { 0 }
        fn get_i128_le(&mut self) -> i128 { 0 }
        fn get_i128_ne(&mut self) -> i128 { 0 }
        fn get_uint(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _: usize) -> u64 { 0 }
        fn get_int(&mut self, _: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
        fn try_get_u8(&mut self) -> Result<u8, TryGetError> { Ok(0) }
        fn try_get_i8(&mut self) -> Result<i8, TryGetError> { Ok(0) }
        fn try_get_u16(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_le(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_u16_ne(&mut self) -> Result<u16, TryGetError> { Ok(0) }
        fn try_get_i16(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_le(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_i16_ne(&mut self) -> Result<i16, TryGetError> { Ok(0) }
        fn try_get_u32(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_le(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_u32_ne(&mut self) -> Result<u32, TryGetError> { Ok(0) }
        fn try_get_i32(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_le(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_i32_ne(&mut self) -> Result<i32, TryGetError> { Ok(0) }
        fn try_get_u64(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_le(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_u64_ne(&mut self) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_i64(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_le(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_i64_ne(&mut self) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_u128(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_le(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_u128_ne(&mut self) -> Result<u128, TryGetError> { Ok(0) }
        fn try_get_i128(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_le(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_i128_ne(&mut self) -> Result<i128, TryGetError> { Ok(0) }
        fn try_get_uint(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf_a = TestBufA { data: vec![1, 2, 3, 4], position: 4 };
    let mut buf_b = TestBufB { data: vec![], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    chain_buf.advance(2);

    assert_eq!(chain_buf.a.remaining(), 0);
    assert_eq!(chain_buf.b.remaining(), 0);
}

