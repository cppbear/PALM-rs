// Answer 0

#[test]
fn test_consume_with_zero_amt() {
    struct MockBuf {
        remaining: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, cnt: usize) {
            self.remaining = self.remaining.saturating_sub(cnt);
        }
        fn has_remaining(&self) -> bool {
            self.remaining > 0
        }
        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
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
        fn get_uint(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_int(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _dst: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
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
        fn try_get_uint(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf = MockBuf { remaining: 10 };
    let mut reader = Reader { buf };

    reader.consume(0);

    assert_eq!(reader.buf.remaining(), 10);
}

#[test]
fn test_consume_with_full_amt() {
    struct MockBuf {
        remaining: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, cnt: usize) {
            self.remaining = self.remaining.saturating_sub(cnt);
        }
        fn has_remaining(&self) -> bool {
            self.remaining > 0
        }
        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
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
        fn get_uint(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_int(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _dst: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
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
        fn try_get_uint(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf = MockBuf { remaining: 10 };
    let mut reader = Reader { buf };

    reader.consume(10);

    assert_eq!(reader.buf.remaining(), 0);
}

#[test]
#[should_panic]
fn test_consume_panic_on_exceeding_amt() {
    struct MockBuf {
        remaining: usize,
    }

    impl Buf for MockBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] {
            &[]
        }
        fn advance(&mut self, cnt: usize) {
            self.remaining = self.remaining.saturating_sub(cnt);
        }
        fn has_remaining(&self) -> bool {
            self.remaining > 0
        }
        fn copy_to_slice(&mut self, _dst: &mut [u8]) {}
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
        fn get_uint(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_le(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_uint_ne(&mut self, _nbytes: usize) -> u64 { 0 }
        fn get_int(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_le(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_int_ne(&mut self, _nbytes: usize) -> i64 { 0 }
        fn get_f32(&mut self) -> f32 { 0.0 }
        fn get_f32_le(&mut self) -> f32 { 0.0 }
        fn get_f32_ne(&mut self) -> f32 { 0.0 }
        fn get_f64(&mut self) -> f64 { 0.0 }
        fn get_f64_le(&mut self) -> f64 { 0.0 }
        fn get_f64_ne(&mut self) -> f64 { 0.0 }
        fn try_copy_to_slice(&mut self, _dst: &mut [u8]) -> Result<(), TryGetError> { Ok(()) }
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
        fn try_get_uint(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_le(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_uint_ne(&mut self, _nbytes: usize) -> Result<u64, TryGetError> { Ok(0) }
        fn try_get_int(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_le(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_int_ne(&mut self, _nbytes: usize) -> Result<i64, TryGetError> { Ok(0) }
        fn try_get_f32(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_le(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f32_ne(&mut self) -> Result<f32, TryGetError> { Ok(0.0) }
        fn try_get_f64(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_le(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn try_get_f64_ne(&mut self) -> Result<f64, TryGetError> { Ok(0.0) }
        fn copy_to_bytes(&mut self, _len: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let mut buf = MockBuf { remaining: 5 };
    let mut reader = Reader { buf };

    reader.consume(10);
}

