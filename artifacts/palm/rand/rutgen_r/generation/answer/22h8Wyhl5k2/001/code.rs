// Answer 0

#[derive(Debug)]
struct RandomState {
    state: u128,
}

impl RandomState {
    const MULTIPLIER: u128 = 6364136223846793005; // Example multiplier constant

    pub fn new(state: u128) -> Self {
        Self { state }
    }

    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = Self::MULTIPLIER;
        let mut cur_plus: u128 = 0;
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
fn test_advance_with_positive_delta() {
    let mut random_state = RandomState::new(12345);
    random_state.advance(1);
    assert!(random_state.state > 12345); // state should change
}

#[test]
fn test_advance_with_large_odd_delta() {
    let mut random_state = RandomState::new(12345);
    random_state.advance(3);
    assert!(random_state.state > 12345); // state should change
}

#[test]
fn test_advance_with_small_odd_delta() {
    let mut random_state = RandomState::new(12345);
    random_state.advance(1);
    assert_ne!(random_state.state, 12345); // state must differ after advancing
}

#[should_panic]
#[test]
fn test_advance_with_zero_delta() {
    let mut random_state = RandomState::new(12345);
    random_state.advance(0); // Should not panic, state should remain the same
    assert_eq!(random_state.state, 12345);
}

