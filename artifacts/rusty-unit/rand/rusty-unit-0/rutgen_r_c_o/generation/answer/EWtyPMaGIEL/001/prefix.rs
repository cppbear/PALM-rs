// Answer 0

#[test]
fn test_next_u64_normal() {
    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u32;

        fn next_u32(&mut self) -> u32 {
            42
        }
    }

    struct MockReseeder;

    impl TryRngCore for MockReseeder {
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), ()> {
            Ok(())
        }
    }

    let inner_rng = MockRng;
    let reseeder = MockReseeder;
    let threshold = 0;
    let bytes_until_reseed = 100;

    let reseeding_core = ReseedingCore {
        inner: inner_rng,
        reseeder,
        threshold,
        bytes_until_reseed,
    };

    let mut rng = ReseedingRng(BlockRng::from(reseeding_core));

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_max_values() {
    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u32;

        fn next_u32(&mut self) -> u32 {
            84
        }
    }

    struct MockReseeder;

    impl TryRngCore for MockReseeder {
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), ()> {
            Ok(())
        }
    }

    let inner_rng = MockRng;
    let reseeder = MockReseeder;
    let threshold = i64::MAX;
    let bytes_until_reseed = u32::MAX as i64;

    let reseeding_core = ReseedingCore {
        inner: inner_rng,
        reseeder,
        threshold,
        bytes_until_reseed,
    };

    let mut rng = ReseedingRng(BlockRng::from(reseeding_core));

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_minimum_threshold() {
    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u32;

        fn next_u32(&mut self) -> u32 {
            21
        }
    }

    struct MockReseeder;

    impl TryRngCore for MockReseeder {
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), ()> {
            Ok(())
        }
    }

    let inner_rng = MockRng;
    let reseeder = MockReseeder;
    let threshold = i64::MIN;
    let bytes_until_reseed = 50;

    let reseeding_core = ReseedingCore {
        inner: inner_rng,
        reseeder,
        threshold,
        bytes_until_reseed,
    };

    let mut rng = ReseedingRng(BlockRng::from(reseeding_core));

    let _result = rng.next_u64();
}

#[test]
fn test_next_u64_with_zero_bytes_until_reseed() {
    struct MockRng;

    impl BlockRngCore for MockRng {
        type Item = u32;

        fn next_u32(&mut self) -> u32 {
            100
        }
    }

    struct MockReseeder;

    impl TryRngCore for MockReseeder {
        fn try_fill_bytes(&mut self, _: &mut [u8]) -> Result<(), ()> {
            Ok(())
        }
    }

    let inner_rng = MockRng;
    let reseeder = MockReseeder;
    let threshold = 10;
    let bytes_until_reseed = 0;

    let reseeding_core = ReseedingCore {
        inner: inner_rng,
        reseeder,
        threshold,
        bytes_until_reseed,
    };

    let mut rng = ReseedingRng(BlockRng::from(reseeding_core));

    let _result = rng.next_u64();
}

