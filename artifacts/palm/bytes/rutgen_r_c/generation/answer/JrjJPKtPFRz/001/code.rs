// Answer 0

#[test]
fn test_reader_read_with_zero_length_buffer() {
    struct TestBuf;

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, _: usize) {}

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }

        fn copy_to_slice(&mut self, _: &mut [u8]) {}

        fn get_u8(&mut self) -> u8 {
            0
        }

        fn get_i8(&mut self) -> i8 {
            0
        }

        fn get_u16(&mut self) -> u16 {
            0
        }

        fn get_u32(&mut self) -> u32 {
            0
        }

        fn get_u64(&mut self) -> u64 {
            0
        }

        fn get_u128(&mut self) -> u128 {
            0
        }

        fn get_i16(&mut self) -> i16 {
            0
        }

        fn get_i32(&mut self) -> i32 {
            0
        }

        fn get_i64(&mut self) -> i64 {
            0
        }

        fn get_i128(&mut self) -> i128 {
            0
        }

        fn get_uint(&mut self, _: usize) -> u64 {
            0
        }

        fn get_int(&mut self, _: usize) -> i64 {
            0
        }

        fn get_f32(&mut self) -> f32 {
            0.0
        }

        fn get_f64(&mut self) -> f64 {
            0.0
        }
    }

    let mut buf = TestBuf;
    let mut dst = vec![0u8; 10];

    let reader = Reader { buf: buf };
    let result = reader.read(&mut dst);

    assert_eq!(result, Ok(0));
}

#[test]
fn test_reader_read_with_full_buffer() {
    struct FullBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl FullBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for FullBuf {
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
            dst[..len].copy_from_slice(&self.data[self.position..self.position + len]);
            self.position += len;
        }

        fn get_u8(&mut self) -> u8 {
            0
        }

        fn get_i8(&mut self) -> i8 {
            0
        }

        fn get_u16(&mut self) -> u16 {
            0
        }

        fn get_u32(&mut self) -> u32 {
            0
        }

        fn get_u64(&mut self) -> u64 {
            0
        }

        fn get_u128(&mut self) -> u128 {
            0
        }

        fn get_i16(&mut self) -> i16 {
            0
        }

        fn get_i32(&mut self) -> i32 {
            0
        }

        fn get_i64(&mut self) -> i64 {
            0
        }

        fn get_i128(&mut self) -> i128 {
            0
        }

        fn get_uint(&mut self, _: usize) -> u64 {
            0
        }

        fn get_int(&mut self, _: usize) -> i64 {
            0
        }

        fn get_f32(&mut self) -> f32 {
            0.0
        }

        fn get_f64(&mut self) -> f64 {
            0.0
        }
    }

    let data = vec![1, 2, 3, 4, 5];
    let mut buf = FullBuf::new(data);
    let mut dst = vec![0u8; 5];

    let mut reader = Reader { buf: buf };
    let result = reader.read(&mut dst);

    assert_eq!(result, Ok(5));
    assert_eq!(dst, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_reader_read_with_partial_buffer() {
    struct PartialBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl PartialBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl Buf for PartialBuf {
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
            dst[..len].copy_from_slice(&self.data[self.position..self.position + len]);
            self.position += len;
        }

        fn get_u8(&mut self) -> u8 {
            0
        }

        fn get_i8(&mut self) -> i8 {
            0
        }

        fn get_u16(&mut self) -> u16 {
            0
        }

        fn get_u32(&mut self) -> u32 {
            0
        }

        fn get_u64(&mut self) -> u64 {
            0
        }

        fn get_u128(&mut self) -> u128 {
            0
        }

        fn get_i16(&mut self) -> i16 {
            0
        }

        fn get_i32(&mut self) -> i32 {
            0
        }

        fn get_i64(&mut self) -> i64 {
            0
        }

        fn get_i128(&mut self) -> i128 {
            0
        }

        fn get_uint(&mut self, _: usize) -> u64 {
            0
        }

        fn get_int(&mut self, _: usize) -> i64 {
            0
        }

        fn get_f32(&mut self) -> f32 {
            0.0
        }

        fn get_f64(&mut self) -> f64 {
            0.0
        }
    }

    let data = vec![1, 2, 3];
    let mut buf = PartialBuf::new(data);
    let mut dst = vec![0u8; 5];

    let mut reader = Reader { buf: buf };
    let result = reader.read(&mut dst);

    assert_eq!(result, Ok(3));
    assert_eq!(dst[..3], vec![1, 2, 3]);
    assert_eq!(dst[3..], vec![0, 0]);
}

