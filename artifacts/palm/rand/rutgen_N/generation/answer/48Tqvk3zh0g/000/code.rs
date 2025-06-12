// Answer 0

#[derive(Debug)]
struct PcgGenerator {
    state: u64,
}

impl PcgGenerator {
    fn new(state: u64) -> Self {
        PcgGenerator { state }
    }

    fn next_u64(&mut self) -> u64 {
        // Simple implementation: Increment the state and return it
        self.state += 1;
        self.state
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

#[test]
fn test_next_u32() {
    let mut generator = PcgGenerator::new(0);
    assert_eq!(generator.next_u32(), 1);
    assert_eq!(generator.next_u32(), 2);
}

#[test]
fn test_next_u32_boundary() {
    let mut generator = PcgGenerator::new(u64::MAX);
    assert_eq!(generator.next_u32(), 0); // Overflow case
    assert_eq!(generator.next_u32(), 1);
}

