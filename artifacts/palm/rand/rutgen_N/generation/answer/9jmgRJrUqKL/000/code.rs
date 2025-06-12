// Answer 0

#[test]
fn test_new_reseeding_rng_valid() {
    struct DummyRsdr;
    impl DummyRsdr {
        type Error = ();
    }

    struct ReseedingCore {
        threshold: u64,
        reseeder: DummyRsdr,
    }

    struct BlockRng {
        core: ReseedingCore,
    }

    struct ReseedingRng(BlockRng);

    impl ReseedingCore {
        fn new(threshold: u64, reseeder: DummyRsdr) -> Result<Self, DummyRsdr::Error> {
            Ok(ReseedingCore { threshold, reseeder })
        }
    }

    impl BlockRng {
        fn new(core: ReseedingCore) -> Self {
            BlockRng { core }
        }
    }

    fn new(threshold: u64, reseeder: DummyRsdr) -> Result<ReseedingRng, DummyRsdr::Error> {
        Ok(ReseedingRng(BlockRng::new(ReseedingCore::new(threshold, reseeder)?)))
    }

    assert!(new(5, DummyRsdr).is_ok());
}

#[test]
#[should_panic]
fn test_new_reseeding_rng_invalid() {
    struct DummyRsdr;
    impl DummyRsdr {
        type Error = ();
    }

    struct ReseedingCore {
        threshold: u64,
        reseeder: DummyRsdr,
    }

    struct BlockRng {
        core: ReseedingCore,
    }

    struct ReseedingRng(BlockRng);

    impl ReseedingCore {
        fn new(_threshold: u64, _reseeder: DummyRsdr) -> Result<Self, DummyRsdr::Error> {
            Err(())
        }
    }

    impl BlockRng {
        fn new(core: ReseedingCore) -> Self {
            BlockRng { core }
        }
    }

    fn new(threshold: u64, reseeder: DummyRsdr) -> Result<ReseedingRng, DummyRsdr::Error> {
        Ok(ReseedingRng(BlockRng::new(ReseedingCore::new(threshold, reseeder)?)))
    }

    let _ = new(5, DummyRsdr);
}

