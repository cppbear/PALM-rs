// Answer 0

#[derive(Default)]
struct TestStruct(u32);

impl TestStruct {
    fn try_fill_bytes(&self, dst: &mut [u8]) -> Result<(), &'static str> {
        if dst.len() < 4 {
            return Err("Buffer too small");
        }
        dst.copy_from_slice(&self.0.to_le_bytes());
        Ok(())
    }
}

impl TestStruct {
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.try_fill_bytes(dst).unwrap();
    }
}

#[test]
fn test_fill_bytes_success() {
    let mut test_struct = TestStruct::default();
    let mut buffer = [0u8; 4];
    test_struct.fill_bytes(&mut buffer);
    assert_eq!(buffer, test_struct.0.to_le_bytes());
}

#[test]
#[should_panic(expected = "Buffer too small")]
fn test_fill_bytes_failure() {
    let mut test_struct = TestStruct::default();
    let mut small_buffer = [0u8; 2];
    test_struct.fill_bytes(&mut small_buffer);
}

