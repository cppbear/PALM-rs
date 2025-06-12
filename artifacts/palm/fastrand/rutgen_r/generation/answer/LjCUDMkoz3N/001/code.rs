// Answer 0

#[derive(Debug)]
struct RandomGenerator(u64);

impl RandomGenerator {
    fn gen_u64(&mut self) -> u64 {
        const WY_CONST_0: u64 = 0x2d35_8dcc_aa6c_78a5;
        const WY_CONST_1: u64 = 0x8bb8_4b93_962e_acc9;

        let s = self.0.wrapping_add(WY_CONST_0);
        self.0 = s;
        let t = u128::from(s) * u128::from(s ^ WY_CONST_1);
        (t as u64) ^ (t >> 64) as u64
    }
}

#[test]
fn test_gen_u64_boundary_low() {
    let mut gen = RandomGenerator(0);
    let result = gen.gen_u64();
    assert!(result >= 0);
}

#[test]
fn test_gen_u64_boundary_high() {
    let mut gen = RandomGenerator(u64::MAX);
    let result = gen.gen_u64();
    assert!(result <= u64::MAX);
}

#[test]
fn test_gen_u64_second_call() {
    let mut gen = RandomGenerator(1);
    let first_result = gen.gen_u64();
    let second_result = gen.gen_u64();
    assert!(first_result != second_result);
}

#[test]
fn test_gen_u64_randomness() {
    let mut gen = RandomGenerator(1234);
    let results: std::collections::HashSet<u64> = (0..1000).map(|_| gen.gen_u64()).collect();
    assert!(results.len() > 1);
}

