// Answer 0

#[derive(Clone, PartialEq, Eq)]
struct TestRng {
    state: u128,
}

impl RngCore for TestRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(MULTIPLIER);
        (self.state as u64) as u32
    }
    
    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(MULTIPLIER);
        self.state as u64
    }
    
    #[inline]
    fn fill_bytes(&mut self, _dest: &mut [u8]) {}
}

#[test]
fn test_next_u32_min_value() {
    let mut rng = TestRng { state: 0 };
    rng.next_u32();
}

#[test]
fn test_next_u32_zero_state() {
    let mut rng = TestRng { state: 0 };
    rng.next_u32();
}

#[test]
fn test_next_u32_small_state() {
    let mut rng = TestRng { state: 1 };
    rng.next_u32();
}

#[test]
fn test_next_u32_large_state() {
    let mut rng = TestRng { state: u128::MAX };
    rng.next_u32();
}

#[test]
fn test_next_u32_random_state() {
    let mut rng = TestRng { state: 123456789012345678901234567890123456 };
    rng.next_u32();
}

#[test]
fn test_next_u32_even_large_state() {
    let mut rng = TestRng { state: 2u128.pow(127) };
    rng.next_u32();
}

#[test]
fn test_next_u32_odd_large_state() {
    let mut rng = TestRng { state: 2u128.pow(127) + 1 };
    rng.next_u32();
}

