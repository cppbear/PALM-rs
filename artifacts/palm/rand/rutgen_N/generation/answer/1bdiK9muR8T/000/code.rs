// Answer 0

#[derive(Default)]
struct MockRng {
    value: u64,
}

impl MockRng {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn next_u64(&mut self) -> u64 {
        self.value
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

#[test]
fn test_next_u32() {
    let mut rng = MockRng::new(0);
    assert_eq!(rng.next_u32(), 0);

    rng.value = u32::MAX as u64;
    assert_eq!(rng.next_u32(), u32::MAX);
}

#[test]
fn test_next_u32_boundary_conditions() {
    let mut rng_min = MockRng::new(0);
    let mut rng_max = MockRng::new(u32::MAX as u64);

    assert_eq!(rng_min.next_u32(), 0);
    assert_eq!(rng_max.next_u32(), u32::MAX);
    
    // Test with a mid-range value
    let mut rng_mid = MockRng::new(0x7FFFFFFF);
    assert_eq!(rng_mid.next_u32(), 0x7FFFFFFF);
}

