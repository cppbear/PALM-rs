// Answer 0

#[test]
fn test_sample_with_valid_rng() {
    struct TestRng(u32);
    
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            let val = self.0;
            self.0 += 1;
            val
        }
    }

    let rng = &mut TestRng(0);
    let result = StandardUniform.sample(rng);
}

#[test]
fn test_sample_with_edge_case_rng_zero() {
    struct TestRng(u32);
    
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
    }

    let rng = &mut TestRng(0);
    let result = StandardUniform.sample(rng);
}

#[test]
fn test_sample_with_edge_case_rng_max() {
    struct TestRng(u32);
    
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            u32::MAX
        }
    }

    let rng = &mut TestRng(0);
    let result = StandardUniform.sample(rng);
}

#[test]
fn test_sample_with_sequential_rng() {
    struct TestRng(u32);
    
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.0 += 1;
            self.0 - 1
        }
    }

    let rng = &mut TestRng(0);
    let result1 = StandardUniform.sample(rng);
    let result2 = StandardUniform.sample(rng);
    let result3 = StandardUniform.sample(rng);
}

#[test]
fn test_sample_with_large_rng_steps() {
    struct TestRng(u32);
    
    impl Rng for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.0 += 256; // stepping by a larger value
            self.0
        }
    }

    let rng = &mut TestRng(0);
    let result = StandardUniform.sample(rng);
}

