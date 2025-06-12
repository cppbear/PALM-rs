// Answer 0

#[derive(Default)]
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }
}

impl RngCore for SimpleRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(2862933555777941757).wrapping_add(3037000493);
        self.state
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        for byte in dst.iter_mut() {
            *byte = self.next_u32() as u8;
        }
    }
}

#[test]
fn test_try_next_u64_success() {
    let mut rng = SimpleRng::new(1);
    let _result = rng.try_next_u64();
}

#[test]
fn test_try_next_u64_alternate_seed() {
    let mut rng = SimpleRng::new(42);
    let _result = rng.try_next_u64();
}

#[test]
fn test_try_next_u64_large_seed() {
    let mut rng = SimpleRng::new(u64::MAX);
    let _result = rng.try_next_u64();
}

#[test]
fn test_try_next_u64_zero_seed() {
    let mut rng = SimpleRng::new(0);
    let _result = rng.try_next_u64();
}

