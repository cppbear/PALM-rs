// Answer 0

#[derive(Default)]
struct TestRngCore {
    value: usize,
}

impl rand_core::RngCore for TestRngCore {
    fn next_u32(&mut self) -> u32 {
        self.value += 1;
        self.value as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.value += 1;
        self.value as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.next_u32() as u8;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        self.fill_bytes(dest);
        Ok(())
    }

    type Error = ();

    fn set_panic_mode(&mut self, _: bool) {}
}

struct R;

impl R {
    fn new(_: usize) -> Self {
        R {}
    }

    fn try_from_rng<S: TryRngCore>(rng: &mut S) -> Result<usize, S::Error> {
        let mut bytes = [0u8; 4];
        rng.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes) as usize)
    }
}

#[test]
fn test_try_from_rng() {
    let mut rng = TestRngCore::default();
    let result = R::try_from_rng(&mut rng);
    assert!(result.is_ok());
}

#[test]
fn test_try_from_rng_boundary() {
    let mut rng = TestRngCore::default();
    for _ in 0..10 {
        let result = R::try_from_rng(&mut rng);
        assert!(result.is_ok());
    }
}

