// Answer 0

#[derive(Debug)]
struct TestRand {
    counter: u32,
}

impl TestRand {
    fn new() -> Self {
        TestRand { counter: 0 }
    }

    fn next_u32(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }
}

impl rand_core::RngCore for TestRand {
    type Error = ();

    fn next_u32(&mut self) -> u32 {
        self.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        unimplemented!()
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

#[test]
fn test_try_next_u32() {
    let mut rng = TestRand::new();
    assert_eq!(rng.try_next_u32().unwrap(), 1);
    assert_eq!(rng.try_next_u32().unwrap(), 2);
    assert_eq!(rng.try_next_u32().unwrap(), 3);
}

#[test]
fn test_try_next_u32_initial() {
    let mut rng = TestRand::new();
    assert_eq!(rng.try_next_u32().unwrap(), 1);
}

