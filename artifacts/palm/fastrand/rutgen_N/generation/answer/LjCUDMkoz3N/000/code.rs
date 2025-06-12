// Answer 0

#[derive(Debug)]
struct RandomGenerator {
    seed: u64,
}

impl RandomGenerator {
    fn new(seed: u64) -> Self {
        RandomGenerator { seed }
    }

    fn gen_u64(&mut self) -> u64 {
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = self.seed.wrapping_add(WY_CONST_0);
        self.seed = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }
}

#[test]
fn test_gen_u64() {
    let mut rng = RandomGenerator::new(0);
    let result = rng.gen_u64();
    assert!(result != 0, "Expected non-zero value");
}

#[test]
fn test_gen_u64_with_different_seeds() {
    let mut rng1 = RandomGenerator::new(1);
    let mut rng2 = RandomGenerator::new(2);
    let result1 = rng1.gen_u64();
    let result2 = rng2.gen_u64();
    assert!(result1 != result2, "Expected different results for different seeds");
}

#[test]
fn test_gen_u64_boundary_conditions() {
    let mut rng = RandomGenerator::new(u64::MAX);
    let result = rng.gen_u64();
    assert!(result != u64::MAX, "Expected result not to be equal to u64::MAX");
}

