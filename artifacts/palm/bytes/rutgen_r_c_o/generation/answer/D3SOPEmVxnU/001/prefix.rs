// Answer 0

#[test]
fn test_flush_empty_buffer() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put<T: super::Buf>(&mut self, _src: T) {}

        fn put_slice(&mut self, _src: &[u8]) {}

        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}

        fn put_u8(&mut self, _n: u8) {}

        fn put_i8(&mut self, _n: i8) {}

        fn put_u16(&mut self, _n: u16) {}

        fn put_u16_le(&mut self, _n: u16) {}

        fn put_u16_ne(&mut self, _n: u16) {}

        fn put_i16(&mut self, _n: i16) {}

        fn put_i16_le(&mut self, _n: i16) {}

        fn put_i16_ne(&mut self, _n: i16) {}

        fn put_u32(&mut self, _n: u32) {}

        fn put_u32_le(&mut self, _n: u32) {}

        fn put_u32_ne(&mut self, _n: u32) {}

        fn put_i32(&mut self, _n: i32) {}

        fn put_i32_le(&mut self, _n: i32) {}

        fn put_i32_ne(&mut self, _n: i32) {}

        fn put_u64(&mut self, _n: u64) {}

        fn put_u64_le(&mut self, _n: u64) {}

        fn put_u64_ne(&mut self, _n: u64) {}

        fn put_i64(&mut self, _n: i64) {}

        fn put_i64_le(&mut self, _n: i64) {}

        fn put_i64_ne(&mut self, _n: i64) {}

        fn put_u128(&mut self, _n: u128) {}

        fn put_u128_le(&mut self, _n: u128) {}

        fn put_u128_ne(&mut self, _n: u128) {}

        fn put_i128(&mut self, _n: i128) {}

        fn put_i128_le(&mut self, _n: i128) {}

        fn put_i128_ne(&mut self, _n: i128) {}

        fn put_uint(&mut self, _n: u64, _nbytes: usize) {}

        fn put_uint_le(&mut self, _n: u64, _nbytes: usize) {}

        fn put_uint_ne(&mut self, _n: u64, _nbytes: usize) {}

        fn put_int(&mut self, _n: i64, _nbytes: usize) {}

        fn put_int_le(&mut self, _n: i64, _nbytes: usize) {}

        fn put_int_ne(&mut self, _n: i64, _nbytes: usize) {}

        fn put_f32(&mut self, _n: f32) {}

        fn put_f32_le(&mut self, _n: f32) {}

        fn put_f32_ne(&mut self, _n: f32) {}

        fn put_f64(&mut self, _n: f64) {}

        fn put_f64_le(&mut self, _n: f64) {}

        fn put_f64_ne(&mut self, _n: f64) {}
    }

    let mut writer = Writer { buf: TestBufMut { remaining: 0 } };
    let _ = writer.flush();
}

#[test]
fn test_flush_with_remaining_data() {
    struct TestBufMut {
        remaining: usize,
    }

    unsafe impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.remaining
        }

        unsafe fn advance_mut(&mut self, _cnt: usize) {}

        fn has_remaining_mut(&self) -> bool {
            self.remaining > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put<T: super::Buf>(&mut self, _src: T) {}

        fn put_slice(&mut self, _src: &[u8]) {}

        fn put_bytes(&mut self, _val: u8, _cnt: usize) {}

        fn put_u8(&mut self, _n: u8) {}

        fn put_i8(&mut self, _n: i8) {}

        fn put_u16(&mut self, _n: u16) {}

        fn put_u16_le(&mut self, _n: u16) {}

        fn put_u16_ne(&mut self, _n: u16) {}

        fn put_i16(&mut self, _n: i16) {}

        fn put_i16_le(&mut self, _n: i16) {}

        fn put_i16_ne(&mut self, _n: i16) {}

        fn put_u32(&mut self, _n: u32) {}

        fn put_u32_le(&mut self, _n: u32) {}

        fn put_u32_ne(&mut self, _n: u32) {}

        fn put_i32(&mut self, _n: i32) {}

        fn put_i32_le(&mut self, _n: i32) {}

        fn put_i32_ne(&mut self, _n: i32) {}

        fn put_u64(&mut self, _n: u64) {}

        fn put_u64_le(&mut self, _n: u64) {}

        fn put_u64_ne(&mut self, _n: u64) {}

        fn put_i64(&mut self, _n: i64) {}

        fn put_i64_le(&mut self, _n: i64) {}

        fn put_i64_ne(&mut self, _n: i64) {}

        fn put_u128(&mut self, _n: u128) {}

        fn put_u128_le(&mut self, _n: u128) {}

        fn put_u128_ne(&mut self, _n: u128) {}

        fn put_i128(&mut self, _n: i128) {}

        fn put_i128_le(&mut self, _n: i128) {}

        fn put_i128_ne(&mut self, _n: i128) {}

        fn put_uint(&mut self, _n: u64, _nbytes: usize) {}

        fn put_uint_le(&mut self, _n: u64, _nbytes: usize) {}

        fn put_uint_ne(&mut self, _n: u64, _nbytes: usize) {}

        fn put_int(&mut self, _n: i64, _nbytes: usize) {}

        fn put_int_le(&mut self, _n: i64, _nbytes: usize) {}

        fn put_int_ne(&mut self, _n: i64, _nbytes: usize) {}

        fn put_f32(&mut self, _n: f32) {}

        fn put_f32_le(&mut self, _n: f32) {}

        fn put_f32_ne(&mut self, _n: f32) {}

        fn put_f64(&mut self, _n: f64) {}

        fn put_f64_le(&mut self, _n: f64) {}

        fn put_f64_ne(&mut self, _n: f64) {}
    }

    let mut writer = Writer { buf: TestBufMut { remaining: 10 } };
    let _ = writer.flush();
}

