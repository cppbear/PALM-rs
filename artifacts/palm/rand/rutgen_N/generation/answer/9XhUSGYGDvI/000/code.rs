// Answer 0

#[derive(Debug)]
struct Pcg {
    state: u128,
    increment: u128,
}

impl Pcg {
    fn step(&mut self) {
        // Simulating some step operation
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(self.increment);
    }

    fn from_state_incr(state: u128, increment: u128) -> Self {
        let mut pcg = Self { state, increment };
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }
}

#[test]
fn test_from_state_incr_boundary() {
    let state: u128 = 0;
    let increment: u128 = 1;
    let pcg = Pcg::from_state_incr(state, increment);
    assert!(pcg.state > 0);
}

#[test]
fn test_from_state_incr_large_numbers() {
    let state: u128 = u128::MAX;
    let increment: u128 = 1;
    let pcg = Pcg::from_state_incr(state, increment);
    assert!(pcg.state < state); // because it wraps around
}

#[test]
fn test_from_state_incr_zero_increment() {
    let state: u128 = 42;
    let increment: u128 = 0;
    let pcg = Pcg::from_state_incr(state, increment);
    assert_eq!(pcg.state, state.wrapping_add(increment).wrapping_mul(6364136223846793005));
}

