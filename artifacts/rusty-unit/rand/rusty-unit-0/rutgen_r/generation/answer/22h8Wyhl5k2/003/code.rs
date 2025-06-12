// Answer 0

#[test]
#[should_panic]
fn test_advance_zero_delta() {
    struct PcgState {
        state: u128,
    }

    const MULTIPLIER: u128 = 6364136223846793005;

    impl PcgState {
        fn new(state: u128) -> Self {
            PcgState { state }
        }

        pub fn advance(&mut self, delta: u128) {
            let mut acc_mult: u128 = 1;
            let mut acc_plus: u128 = 0;
            let mut cur_mult = MULTIPLIER;
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

    let mut state = PcgState::new(42);
    state.advance(0); // This should trigger a panic due to the mdelta condition
}

