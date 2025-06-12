// Answer 0

#[derive(Default)]
struct Pcg {
    state: u64,
}

impl Pcg {
    fn next_u64(&mut self) -> u64 {
        // Simplified version of a next_u64 function for demonstration purpose.
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

#[test]
fn test_next_u32() {
    let mut pcg = Pcg::default();
    let value = pcg.next_u32();
    assert!(value <= u32::MAX);
}

#[test]
fn test_next_u32_reproducibility() {
    let mut pcg1 = Pcg::default();
    let mut pcg2 = Pcg::default();
    
    let value1 = pcg1.next_u32();
    let value2 = pcg2.next_u32();

    assert_eq!(value1, value2);
}

#[test]
fn test_next_u32_state_change() {
    let mut pcg = Pcg::default();
    let first_value = pcg.next_u32();
    let second_value = pcg.next_u32();
    
    assert_ne!(first_value, second_value);
}

