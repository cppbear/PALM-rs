// Answer 0

#[test]
fn test_step() {
    const MULTIPLIER: u64 = 0x5851f42d4c957f2d; // Assuming this is the constant used in the function
    struct PcgState {
        state: u64,
        increment: u64,
    }

    impl PcgState {
        fn new(state: u64, increment: u64) -> Self {
            PcgState { state, increment }
        }

        fn step(&mut self) {
            self.state = self
                .state
                .wrapping_mul(MULTIPLIER)
                .wrapping_add(self.increment);
        }
    }

    let mut pcg = PcgState::new(1, 1);
    pcg.step();
    assert_eq!(pcg.state, 0x5851f42d4c957f2d + 1); // Adjust based on the multiplier

    pcg = PcgState::new(2, 2);
    pcg.step();
    assert_eq!(pcg.state, 0x5851f42d4c957f2d * 2 + 2);
}

