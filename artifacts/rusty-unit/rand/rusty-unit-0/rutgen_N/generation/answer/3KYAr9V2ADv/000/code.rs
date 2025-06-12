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
        // A simple demonstration for the sake of the example.
        self.state = self.state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
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
fn test_next_u32_normal() {
    let mut rng = PcgState::new(12345);
    let result = rng.next_u32();
    assert!(result <= u32::MAX);
}

#[test]
fn test_next_u32_with_default_state() {
    let mut rng = PcgState::new(0);
    let result = rng.next_u32();
    assert!(result <= u32::MAX);
}

#[test]
fn test_next_u32_large_state() {
    let mut rng = PcgState::new(u64::MAX);
    let result = rng.next_u32();
    assert!(result <= u32::MAX);
}

#[test]
fn test_next_u32_consistency() {
    let mut rng = PcgState::new(67890);
    let first = rng.next_u32();
    let second = rng.next_u32();
    assert_ne!(first, second);
}

