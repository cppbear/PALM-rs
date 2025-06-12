// Answer 0

#[derive(Debug)]
struct MockGenerator {
    value: u64,
}

impl MockGenerator {
    fn new(value: u64) -> Self {
        MockGenerator { value }
    }

    fn try_next_u64(&mut self) -> Result<u64, &'static str> {
        if self.value == 0 {
            Err("Panic: Value is zero!")
        } else {
            let next_value = self.value;
            self.value = 0; // After the first call, set to zero to trigger panic on next call
            Ok(next_value)
        }
    }
}

struct RandCore(MockGenerator);

impl RandCore {
    fn next_u64(&mut self) -> u64 {
        self.0.try_next_u64().unwrap()
    }
}

#[test]
fn test_next_u64_success() {
    let mut generator = RandCore(MockGenerator::new(42));
    let result = generator.next_u64();
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "Panic: Value is zero!")]
fn test_next_u64_panics() {
    let mut generator = RandCore(MockGenerator::new(0));
    generator.next_u64();
}

#[test]
#[should_panic(expected = "Panic: Value is zero!")]
fn test_next_u64_panics_after_success() {
    let mut generator = RandCore(MockGenerator::new(100));
    let _ = generator.next_u64();
    // This will trigger the panic in the next call
    generator.next_u64();
}

