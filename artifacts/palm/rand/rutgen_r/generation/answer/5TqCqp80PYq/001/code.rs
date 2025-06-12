// Answer 0

#[derive(Debug)]
struct TestGenerator(u32);

impl TestGenerator {
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), &'static str> {
        if dst.len() > self.0 as usize {
            return Err("Buffer too large");
        }
        for i in 0..dst.len() {
            dst[i] = (self.0 + i as u32) as u8;
        }
        Ok(())
    }
}

impl TestGenerator {
    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.try_fill_bytes(dst).unwrap()
    }
}

#[test]
fn test_fill_bytes_small_buffer() {
    let mut generator = TestGenerator(5);
    let mut buffer = [0u8; 3];
    generator.fill_bytes(&mut buffer);
    assert_eq!(buffer, [5, 6, 7]);
}

#[test]
fn test_fill_bytes_exact_buffer() {
    let mut generator = TestGenerator(5);
    let mut buffer = [0u8; 5];
    generator.fill_bytes(&mut buffer);
    assert_eq!(buffer, [5, 6, 7, 8, 9]);
}

#[should_panic(expected = "Buffer too large")]
#[test]
fn test_fill_bytes_large_buffer() {
    let mut generator = TestGenerator(5);
    let mut buffer = [0u8; 6];
    generator.fill_bytes(&mut buffer);  // This should panic
}

#[test]
fn test_fill_bytes_zero_buffer() {
    let mut generator = TestGenerator(5);
    let mut buffer = [];
    generator.fill_bytes(&mut buffer);  // Should do nothing and not panic
}

