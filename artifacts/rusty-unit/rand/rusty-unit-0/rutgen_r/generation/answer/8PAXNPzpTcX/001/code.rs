// Answer 0

#[derive(Debug)]
struct TestRandCore(u32);

impl TestRandCore {
    fn try_next_u32(&mut self) -> Result<u32, &'static str> {
        if self.0 == 0 {
            Err("Panic condition: zero value")
        } else {
            self.0 -= 1;
            Ok(self.0)
        }
    }
}

impl TestRandCore {
    fn next_u32(&mut self) -> u32 {
        self.try_next_u32().unwrap()
    }
}

#[test]
fn test_next_u32_non_zero() {
    let mut rng = TestRandCore(5);
    assert_eq!(rng.next_u32(), 4);
    assert_eq!(rng.next_u32(), 3);
    assert_eq!(rng.next_u32(), 2);
    assert_eq!(rng.next_u32(), 1);
    assert_eq!(rng.next_u32(), 0);
}

#[should_panic(expected = "Panic condition: zero value")]
#[test]
fn test_next_u32_zero() {
    let mut rng = TestRandCore(0);
    rng.next_u32(); // This should trigger a panic
}

