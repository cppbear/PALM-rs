// Answer 0

#[test]
fn test_from_seed_zero() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 8];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let seed: [u8; 8] = [0; 8];
    let rng = TestRng::from_seed(seed);
}

#[test]
fn test_from_seed_maximum() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 8];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let seed: [u8; 8] = [255; 8];
    let rng = TestRng::from_seed(seed);
}

#[test]
fn test_from_seed_smallest_non_zero() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 8];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let seed: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
    let rng = TestRng::from_seed(seed);
}

#[test]
fn test_from_seed_large_value() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 8];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let seed: [u8; 8] = [0xFF; 8];
    let rng = TestRng::from_seed(seed);
}

#[test]
fn test_from_seed_edge_case() {
    struct TestRng;
    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
        fn next_u64(&mut self) -> u64 {
            0
        }
        fn fill_bytes(&mut self, _dst: &mut [u8]) {}
    }

    impl SeedableRng for TestRng {
        type Seed = [u8; 8];
        fn from_seed(seed: Self::Seed) -> Self {
            TestRng
        }
    }

    let seed: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let rng = TestRng::from_seed(seed);
}

