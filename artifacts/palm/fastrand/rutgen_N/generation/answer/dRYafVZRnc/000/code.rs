// Answer 0

#[derive(Default)]
struct RandomGenerator;

impl RandomGenerator {
    fn gen_u32(&mut self) -> u32 {
        // Simple random implementation for testing
        let seed = 12345; // Fixed seed for reproducibility
        seed.wrapping_mul(1103515245).wrapping_add(12345) % u32::MAX
    }

    fn gen_mod_u32(&mut self, n: u32) -> u32 {
        let mut r = self.gen_u32();
        let mut hi = mul_high_u32(r, n);
        let mut lo = r.wrapping_mul(n);
        if lo < n {
            let t = n.wrapping_neg() % n;
            while lo < t {
                r = self.gen_u32();
                hi = mul_high_u32(r, n);
                lo = r.wrapping_mul(n);
            }
        }
        hi
    }
}

fn mul_high_u32(x: u32, y: u32) -> u32 {
    (x as u64 * y as u64 >> 32) as u32
}

#[test]
fn test_gen_mod_u32_zero() {
    let mut rng = RandomGenerator::default();
    let result = rng.gen_mod_u32(1);
    assert_eq!(result, 0);
}

#[test]
fn test_gen_mod_u32_non_zero() {
    let mut rng = RandomGenerator::default();
    let result = rng.gen_mod_u32(10);
    assert!(result < 10);
}

#[test]
fn test_gen_mod_u32_large() {
    let mut rng = RandomGenerator::default();
    let result = rng.gen_mod_u32(u32::MAX);
    assert!(result < u32::MAX);
}


