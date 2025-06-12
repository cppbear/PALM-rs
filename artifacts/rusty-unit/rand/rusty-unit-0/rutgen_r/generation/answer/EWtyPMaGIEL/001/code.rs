// Answer 0

#[test]
fn test_next_u64() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let result = self.0;
            self.0 += 1; // Simulate the next number by incrementing
            result
        }
    }
    
    let mut rng = MockRng(0);
    
    assert_eq!(rng.next_u64(), 0);
    assert_eq!(rng.next_u64(), 1);
    assert_eq!(rng.next_u64(), 2);
}

#[test]
fn test_next_u64_large_value() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let result = self.0;
            self.0 += 1;
            result
        }
    }
    
    let mut rng = MockRng(u64::MAX);
    
    assert_eq!(rng.next_u64(), u64::MAX);
}

#[test]
fn test_next_u64_multiple_calls() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            let result = self.0;
            self.0 += 2; // Simulating a different pattern for diversity
            result
        }
    }
    
    let mut rng = MockRng(0);
    
    assert_eq!(rng.next_u64(), 0);
    assert_eq!(rng.next_u64(), 2);
    assert_eq!(rng.next_u64(), 4);
}

