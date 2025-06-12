// Answer 0

#[derive(Default)]
struct Pcg128 {
    state: u64,
}

impl Pcg128 {
    fn next_u64(&mut self) -> u64 {
        // Simple linear congruential generator for demonstration
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

#[test]
fn test_next_u32() {
    let mut rng = Pcg128::default();
    
    let output1 = rng.next_u32();
    assert!(output1 <= u32::MAX);
    
    let output2 = rng.next_u32();
    assert!(output2 <= u32::MAX);
    
    let output3 = rng.next_u32();
    assert!(output3 <= u32::MAX);
    
    assert_ne!(output1, output2);
    assert_ne!(output2, output3);
    assert_ne!(output1, output3);
}

#[test]
fn test_next_u32_with_zero_state() {
    let mut rng = Pcg128 { state: 0 };
    
    let output = rng.next_u32();
    assert_eq!(output, 1);
}

#[test]
fn test_next_u32_large_states() {
    let mut rng = Pcg128 { state: u64::MAX };
    
    let output = rng.next_u32();
    assert!(output <= u32::MAX);
}

