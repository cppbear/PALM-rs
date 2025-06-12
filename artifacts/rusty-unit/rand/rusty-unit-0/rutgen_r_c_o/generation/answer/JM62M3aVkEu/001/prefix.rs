// Answer 0

#[derive(Default)]
struct TestRng {
    bytes: Vec<u8>,
    index: usize,
}

impl RngCore for TestRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let len = dest.len().min(self.bytes.len() - self.index);
        dest[..len].copy_from_slice(&self.bytes[self.index..self.index + len]);
        self.index += len;
    }

    fn take_rng(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            index: self.index,
        }
    }

    fn seed_from_u64(&mut self, _seed: u64) {
        // No op
    }
}

#[test]
fn test_next_u64_via_fill_with_valid_rng() {
    let mut rng = TestRng { bytes: vec![1, 0, 0, 0, 0, 0, 0, 0], index: 0 };
    let result = next_u64_via_fill(&mut rng);
}

#[test]
fn test_next_u64_via_fill_with_zeroes() {
    let mut rng = TestRng { bytes: vec![0; 8], index: 0 };
    let result = next_u64_via_fill(&mut rng);
}

#[test]
fn test_next_u64_via_fill_with_random_bytes() {
    let mut rng = TestRng { bytes: vec![42, 43, 44, 45, 46, 47, 48, 49], index: 0 };
    let result = next_u64_via_fill(&mut rng);
}

#[test]
fn test_next_u64_via_fill_with_monotonic_increase() {
    let bytes = (0u8..8).collect::<Vec<_>>();
    let mut rng = TestRng { bytes, index: 0 };
    let result = next_u64_via_fill(&mut rng);
}

#[should_panic]
fn test_next_u64_via_fill_with_underflow_index() {
    let mut rng = TestRng { bytes: vec![1, 1, 1, 1, 1, 1, 1, 1], index: 8 };
    let result = next_u64_via_fill(&mut rng);
}

#[test]
fn test_next_u64_via_fill_with_exact_eight_bytes() {
    let mut rng = TestRng { bytes: vec![255, 254, 253, 252, 251, 250, 249, 248], index: 0 };
    let result = next_u64_via_fill(&mut rng);
}

