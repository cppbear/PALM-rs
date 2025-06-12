// Answer 0

#[test]
fn test_get_ref() {
    use std::vec::Vec;
    struct TestBufMut {
        vec: Vec<u8>,
    }

    impl BufMut for TestBufMut {
        fn remaining_mut(&self) -> usize {
            self.vec.capacity() - self.vec.len()
        }

        unsafe fn advance_mut(&mut self, cnt: usize) {
            self.vec.set_len(self.vec.len() + cnt);
        }

        fn has_remaining_mut(&self) -> bool {
            self.remaining_mut() > 0
        }

        fn chunk_mut(&mut self) -> &mut UninitSlice {
            unimplemented!()
        }

        fn put<T: super::Buf>(&mut self, _src: T) {
            unimplemented!()
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.vec.extend_from_slice(src);
        }

        fn put_bytes(&mut self, _val: u8, _cnt: usize) {
            unimplemented!()
        }

        fn put_u8(&mut self, n: u8) {
            self.vec.push(n);
        }

        fn put_i8(&mut self, _n: i8) { unimplemented!() }
        fn put_u16(&mut self, _n: u16) { unimplemented!() }
        fn put_u16_le(&mut self, _n: u16) { unimplemented!() }
        fn put_u16_ne(&mut self, _n: u16) { unimplemented!() }
        fn put_i16(&mut self, _n: i16) { unimplemented!() }
        fn put_i16_le(&mut self, _n: i16) { unimplemented!() }
        fn put_i16_ne(&mut self, _n: i16) { unimplemented!() }
        fn put_u32(&mut self, _n: u32) { unimplemented!() }
        fn put_u32_le(&mut self, _n: u32) { unimplemented!() }
        fn put_u32_ne(&mut self, _n: u32) { unimplemented!() }
        fn put_i32(&mut self, _n: i32) { unimplemented!() }
        fn put_i32_le(&mut self, _n: i32) { unimplemented!() }
        fn put_i32_ne(&mut self, _n: i32) { unimplemented!() }
        fn put_u64(&mut self, _n: u64) { unimplemented!() }
        fn put_u64_le(&mut self, _n: u64) { unimplemented!() }
        fn put_u64_ne(&mut self, _n: u64) { unimplemented!() }
        fn put_i64(&mut self, _n: i64) { unimplemented!() }
        fn put_i64_le(&mut self, _n: i64) { unimplemented!() }
        fn put_i64_ne(&mut self, _n: i64) { unimplemented!() }
        fn put_u128(&mut self, _n: u128) { unimplemented!() }
        fn put_u128_le(&mut self, _n: u128) { unimplemented!() }
        fn put_u128_ne(&mut self, _n: u128) { unimplemented!() }
        fn put_i128(&mut self, _n: i128) { unimplemented!() }
        fn put_i128_le(&mut self, _n: i128) { unimplemented!() }
        fn put_i128_ne(&mut self, _n: i128) { unimplemented!() }
        fn put_uint(&mut self, _n: u64, _nbytes: usize) { unimplemented!() }
        fn put_uint_le(&mut self, _n: u64, _nbytes: usize) { unimplemented!() }
        fn put_uint_ne(&mut self, _n: u64, _nbytes: usize) { unimplemented!() }
        fn put_int(&mut self, _n: i64, _nbytes: usize) { unimplemented!() }
        fn put_int_le(&mut self, _n: i64, _nbytes: usize) { unimplemented!() }
        fn put_int_ne(&mut self, _n: i64, _nbytes: usize) { unimplemented!() }
        fn put_f32(&mut self, _n: f32) { unimplemented!() }
        fn put_f32_le(&mut self, _n: f32) { unimplemented!() }
        fn put_f32_ne(&mut self, _n: f32) { unimplemented!() }
        fn put_f64(&mut self, _n: f64) { unimplemented!() }
        fn put_f64_le(&mut self, _n: f64) { unimplemented!() }
        fn put_f64_ne(&mut self, _n: f64) { unimplemented!() }
    }

    let buf = TestBufMut { vec: Vec::with_capacity(1024) };
    let writer = Writer { buf };

    assert_eq!(1024, writer.get_ref().remaining_mut());
}

