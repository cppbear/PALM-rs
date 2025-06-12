// Answer 0

#[derive(Debug)]
struct PcgState {
    state: u64,
}

impl PcgState {
    fn new(state: u64) -> Self {
        PcgState { state }
    }

    fn step(&mut self) {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
    }

    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        const ROTATE: u32 = 59;
        const XSHIFT: u32 = 18;
        const SPARE: u32 = 27;

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }
}

#[test]
fn test_next_u32() {
    let mut pcg = PcgState::new(0x1234567890abcdef);

    // First output
    let output1 = pcg.next_u32();
    assert!(output1 <= 0xFFFFFFFF);

    // Second output
    let output2 = pcg.next_u32();
    assert!(output2 <= 0xFFFFFFFF);

    // Check for different initial state
    let mut pcg2 = PcgState::new(0x0);
    let output3 = pcg2.next_u32();
    assert!(output3 <= 0xFFFFFFFF);
    
    let mut pcg3 = PcgState::new(u64::MAX);
    let output4 = pcg3.next_u32();
    assert!(output4 <= 0xFFFFFFFF);
}

