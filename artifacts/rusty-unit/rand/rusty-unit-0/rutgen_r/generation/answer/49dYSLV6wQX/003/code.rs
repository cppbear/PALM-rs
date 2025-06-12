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

    pub fn advance(&mut self, delta: u128) {
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
fn test_advance_with_zero_delta() {
    let mut pgc_state = PcgState::new(42, 54);
    pgc_state.advance(0);
    assert_eq!(pgc_state.state, 42); // State should remain unchanged
}

#[test]
#[should_panic]
fn test_advance_with_negative_delta() {
    let mut pgc_state = PcgState::new(42, 54);
    pgc_state.advance(-1i128 as u128); // Should panic as we use a negative delta 
}

