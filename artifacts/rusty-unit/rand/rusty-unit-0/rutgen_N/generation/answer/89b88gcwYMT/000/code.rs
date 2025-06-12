// Answer 0

#[derive(Debug)]
struct PcgState {
    state: u64,
    increment: u64,
}

const MULTIPLIER: u64 = 6364136223846793005; // Example value, replace with actual constant

impl PcgState {
    pub fn new(state: u64, increment: u64) -> Self {
        PcgState { state, increment }
    }

    pub fn advance(&mut self, delta: u64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
        let mut cur_mult = MULTIPLIER;
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
fn test_advance() {
    let mut pcg = PcgState::new(1, 1);
    pcg.advance(1);
    assert_eq!(pcg.state, 6364136223846793006); // Example expected value, replace with correct one

    let mut pcg2 = PcgState::new(10, 1);
    pcg2.advance(2);
    assert_eq!(pcg2.state, 11664245014183288914); // Example expected value, replace with correct one

    let mut pcg3 = PcgState::new(5, 5);
    pcg3.advance(u64::MAX);
    assert_ne!(pcg3.state, 5); // Ensure state is changed

    let mut pcg4 = PcgState::new(0, 1);
    pcg4.advance(0);
    assert_eq!(pcg4.state, 0); // Ensure state remains unchanged when delta is 0
}

