// Answer 0

#[test]
fn test_reseeding_core_new_threshold_zero() {
    struct DummyRng;
    impl SeedableRng for DummyRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {}
    }
    impl BlockRngCore for DummyRng {
        type Results = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, R::Error> { Ok(DummyRng) }
    }
    struct DummyReseeder;
    impl TryRngCore for DummyReseeder {
        type Error = ();
        fn try_fill_bytes(&mut self, _buf: &mut [u8]) -> Result<(), Self::Error> { Ok(()) }
    }

    let reseeder = DummyReseeder;
    let result = ReseedingCore::<DummyRng, DummyReseeder>::new(0, reseeder);
}

#[test]
fn test_reseeding_core_new_threshold_max() {
    struct DummyRng;
    impl SeedableRng for DummyRng {
        type Seed = [u8; 32];
        fn from_seed(seed: Self::Seed) -> Self {}
    }
    impl BlockRngCore for DummyRng {
        type Results = ();
        fn try_from_rng<R: RngCore>(rng: &mut R) -> Result<Self, R::Error> { Ok(DummyRng) }
    }
    struct DummyReseeder;
    impl TryRngCore for DummyReseeder {
        type Error = ();
        fn try_fill_bytes(&mut self, _buf: &mut [u8]) -> Result<(), Self::Error> { Ok(()) }
    }

    let reseeder = DummyReseeder;
    let result = ReseedingCore::<DummyRng, DummyReseeder>::new(u64::MAX, reseeder);
}

