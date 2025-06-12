// Answer 0

#[derive(Debug)]
struct PcgState {
    state: u128,
    increment: u128,
}

impl PcgState {
    const MULTIPLIER: u128 = 6364136223846793005;

    fn new(state: u128, increment: u128) -> Self {
        Self { state, increment }
    }

    pub fn advance(&mut self, delta: u128) {
        let mut acc_mult: u128 = 1;
        let mut acc_plus: u128 = 0;
        let mut cur_mult = Self::MULTIPLIER;
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
fn test_advance_positive_delta() {
    let mut pcg = PcgState::new(42, 54);
    pcg.advance(1);
    assert!(pcg.state != 42); // Check that state has changed.

    pcg.advance(3);
    assert!(pcg.state != 0); // Ensure state is valid after advancing with a larger delta.
}

#[test]
fn test_advance_large_delta() {
    let mut pcg = PcgState::new(1, 2);
    pcg.advance(1_000_000_000_000);
    assert!(pcg.state != 1); // Check that state has changed from the initial value.
}

#[test]
#[should_panic]
fn test_advance_zero_delta() {
    let mut pcg = PcgState::new(10, 20);
    pcg.advance(0); // This should panic as the condition mdelta > 0 will not hold.
}

