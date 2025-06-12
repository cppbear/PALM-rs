// Answer 0

#[derive(Default)]
struct TestRng {
    value_u64: u64,
    value_u32: u32,
}

impl rand_core::RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        self.value_u32
    }

    fn next_u64(&mut self) -> u64 {
        self.value_u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(0);
    }
    
    fn discard(&mut self, _count: usize) {}
}

#[test]
fn test_fill_bytes_via_next_zero_length() {
    let mut rng = TestRng::default();
    let mut dest: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest.len(), 0);
}

#[test]
fn test_fill_bytes_via_next_four_bytes() {
    let mut rng = TestRng { value_u32: 0x12345678, value_u64: 0 };
    let mut dest: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [0x78, 0x56, 0x34, 0x12]);
}

#[test]
fn test_fill_bytes_via_next_more_than_four_bytes() {
    let mut rng = TestRng { value_u64: 0x0123456789ABCDEF };
    let mut dest: [u8; 5] = [0; 5];
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest[..5], [0xEF, 0xCD, 0xAB, 0x89, 0x67]);
}

