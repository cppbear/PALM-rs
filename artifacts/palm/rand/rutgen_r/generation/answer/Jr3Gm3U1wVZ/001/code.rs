// Answer 0

#[derive(Debug)]
struct Pcg128 {
    state: u64,
    increment: u64,
}

impl Pcg128 {
    const MULTIPLIER: u64 = 0x5851f42d4c957f2d; // Example constant multiplier

    fn new(state: u64, increment: u64) -> Self {
        Pcg128 { state, increment }
    }

    fn step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(self.increment);
    }
}

#[test]
fn test_step_basic() {
    let mut pcg = Pcg128::new(1, 1);
    pcg.step();
    assert!(pcg.state > 1); // Check if state is updated
}

#[test]
fn test_step_with_large_state() {
    let mut pcg = Pcg128::new(u64::MAX, 1);
    pcg.step();
    assert!(pcg.state < u64::MAX); // Ensure state wraps around
}

#[test]
fn test_step_with_large_increment() {
    let mut pcg = Pcg128::new(1, u64::MAX);
    pcg.step();
    assert!(pcg.state > 1); // Verify if state is updated correctly
}

#[test]
fn test_step_with_zero_state() {
    let mut pcg = Pcg128::new(0, 0);
    pcg.step();
    assert_eq!(pcg.state, 0); // Validate if state remains zero as increment is zero
}

#[test]
fn test_step_with_negative_increment() {
    // Rust does not allow negative values for u64, testing boundary condition with maximum
    let mut pcg = Pcg128::new(10, !0);
    pcg.step();
    assert!(pcg.state < 10); // Check wrapping behavior
}

