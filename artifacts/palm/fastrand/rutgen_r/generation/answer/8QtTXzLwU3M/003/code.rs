// Answer 0

#[derive(Debug)]
struct TestRand {
    state: u64,
}

impl TestRand {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    
    fn gen_u64(&mut self) -> u64 {
        // Simple linear congruential generator for tests
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(123456789);
        self.state
    }
}

#[test]
fn test_fill_empty_slice() {
    let mut rng = TestRand::new(42);
    let mut slice: &mut [u8] = &mut [];
    rng.fill(slice);
    assert!(slice.is_empty());
}

#[test]
fn test_fill_small_slice() {
    let mut rng = TestRand::new(42);
    let mut slice: &mut [u8] = &mut [0; 1]; // only 1 byte
    rng.fill(slice);
    assert_eq!(slice.len(), 1);
}

#[test]
fn test_fill_exact_slice() {
    let mut rng = TestRand::new(42);
    let mut slice: &mut [u8] = &mut [0; 8]; // exactly one chunk of 8 bytes
    rng.fill(slice);
    assert_eq!(slice.len(), 8);
}

#[test]
fn test_fill_large_slice() {
    let mut rng = TestRand::new(42);
    let mut slice: &mut [u8] = &mut [0; 15]; // 15 bytes, so 1 chunk of 8 and 1 remainder of 7
    rng.fill(slice);
    assert_eq!(slice.len(), 15);
    assert_eq!(slice[0..8], rng.gen_u64().to_ne_bytes());
    assert_eq!(&slice[8..15], &rng.gen_u64().to_ne_bytes()[..7]);
}

#[test]
#[should_panic]
fn test_fill_slice_with_non_multiple_length() {
    let mut rng = TestRand::new(42);
    let mut slice: &mut [u8] = &mut [0; 9]; // 9 bytes. It should panic due to chunking
    rng.fill(slice);
}

