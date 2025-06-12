// Answer 0

#[derive(Default)]
struct MockRng {
    value: u32,
}

impl MockRng {
    fn new(value: u32) -> Self {
        MockRng { value }
    }

    fn next_u32(&mut self) -> u32 {
        self.value
    }
}

impl rand_core::RngCore for MockRng {
    type Error = ();
    
    fn next_u32(&mut self) -> u32 {
        self.value
    }

    fn set_sequence(&mut self, _seed: &mut [u8]) {}

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.next_u32())
    }
}

#[test]
fn test_try_next_u32_success() {
    let mut rng = MockRng::new(42);
    let result = rng.try_next_u32();
    assert_eq!(result, Ok(42));
}

#[test]
fn test_try_next_u32_boundary() {
    let mut rng_zero = MockRng::new(0);
    let result_zero = rng_zero.try_next_u32();
    assert_eq!(result_zero, Ok(0));

    let mut rng_max = MockRng::new(u32::MAX);
    let result_max = rng_max.try_next_u32();
    assert_eq!(result_max, Ok(u32::MAX));
}

