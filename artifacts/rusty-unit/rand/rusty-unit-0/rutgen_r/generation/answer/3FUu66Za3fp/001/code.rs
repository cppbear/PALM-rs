// Answer 0

#[derive(Debug)]
struct MockRng(u64);

impl MockRng {
    fn try_next_u64(&mut self) -> Result<u64, &'static str> {
        if self.0 == 0 {
            Err("RNG exhausted")
        } else {
            self.0 -= 1;
            Ok(self.0)
        }
    }
}

struct MyRng(MockRng);

impl MyRng {
    fn next_u64(&mut self) -> u64 {
        self.0.try_next_u64().unwrap()
    }
}

#[test]
fn test_next_u64_success() {
    let mut rng = MyRng(MockRng(10));
    assert_eq!(rng.next_u64(), 9);
    assert_eq!(rng.next_u64(), 8);
}

#[test]
#[should_panic(expected = "RNG exhausted")]
fn test_next_u64_panic() {
    let mut rng = MyRng(MockRng(0));
    rng.next_u64();
}

#[test]
fn test_next_u64_boundary() {
    let mut rng = MyRng(MockRng(1));
    assert_eq!(rng.next_u64(), 0);
    assert!(std::panic::catch_unwind(|| rng.next_u64()).is_err());
}

