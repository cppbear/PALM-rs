// Answer 0

#[derive(Default)]
struct TestCore {
    results: [u32; 10],
}

impl BlockRngCore for TestCore {
    type Item = u32;
    type Results = [u32; 10];

    fn generate(&mut self, results: &mut Self::Results) {
        results.copy_from_slice(&self.results);
    }
}

#[test]
fn test_fill_bytes_empty_dest() {
    let mut core = TestCore::default();
    let mut rng = BlockRng::new(core);
    let mut dest: [u8; 0] = [];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_small_dest() {
    let mut core = TestCore { results: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
    let mut rng = BlockRng::new(core);
    let mut dest: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_exact_fit() {
    let mut core = TestCore { results: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
    let mut rng = BlockRng::new(core);
    let mut dest: [u8; 40] = [0; 40];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_larger_dest() {
    let mut core = TestCore { results: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
    let mut rng = BlockRng::new(core);
    let mut dest: [u8; 80] = [0; 80];
    rng.fill_bytes(&mut dest);
}

#[test]
fn test_fill_bytes_full_dest() {
    let mut core = TestCore { results: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] };
    let mut rng = BlockRng::new(core);
    let mut dest: [u8; 32] = [0; 32];
    rng.fill_bytes(&mut dest);
}

