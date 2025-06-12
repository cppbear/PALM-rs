// Answer 0

#[derive(Debug)]
struct PcgState {
    state: u128,
    increment: u128,
}

impl PcgState {
    fn new(state: u128, increment: u128) -> Self {
        PcgState { state, increment }
    }

    fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = 6364136223846793005; // MULTIPLIER
        let mut cur_plus = self.increment;
        let mut mdelta = delta;

        while mdelta > 0 {
            if (mdelta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            mdelta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }
}

#[test]
fn test_advance_zero_delta() {
    let mut pcg = PcgState::new(42, 54);
    pcg.advance(0);
    assert_eq!(pcg.state, 42);
}

#[test]
fn test_advance_one_delta() {
    let mut pcg = PcgState::new(42, 54);
    pcg.advance(1);
    assert_ne!(pcg.state, 42);
}

#[test]
fn test_advance_large_delta() {
    let mut pcg = PcgState::new(42, 54);
    let initial_state = pcg.state;
    pcg.advance(10000);
    assert_ne!(pcg.state, initial_state);
}

#[test]
fn test_advance_negative_delta() {
    let mut pcg = PcgState::new(42, 54);
    pcg.advance(u128::MAX);
    assert_ne!(pcg.state, 42);
}

