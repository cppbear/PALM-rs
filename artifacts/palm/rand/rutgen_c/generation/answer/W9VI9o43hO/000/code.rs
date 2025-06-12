// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_next_u64() {
        struct TestRng {
            state: u64,
        }
        
        impl RngCore for TestRng {
            fn next_u32(&mut self) -> u32 {
                self.state as u32
            }
            fn next_u64(&mut self) -> u64 {
                self.state
            }
            fn fill_bytes(&mut self, _dest: &mut [u8]) {
                // No-op for test
            }
        }
        
        let mut rng = TestRng { state: 42 };
        let small_rng = SmallRng(rng);
        let result = small_rng.next_u64();
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_next_u64_with_different_state() {
        struct TestRng {
            state: u64,
        }
        
        impl RngCore for TestRng {
            fn next_u32(&mut self) -> u32 {
                self.state as u32
            }
            fn next_u64(&mut self) -> u64 {
                self.state
            }
            fn fill_bytes(&mut self, _dest: &mut [u8]) {
                // No-op for test
            }
        }
        
        let mut rng = TestRng { state: 100 };
        let small_rng = SmallRng(rng);
        let result = small_rng.next_u64();
        assert_eq!(result, 100);
    }
}

