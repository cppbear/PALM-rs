// Answer 0

#[test]
fn test_next_u64() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            self.0 += 1; // Incrementing each call for predictable output
            self.0
        }
    }
    
    let mut rng = MockRng(0);
    let first = rng.next_u64();
    let second = rng.next_u64();
    
    assert_eq!(first, 1);
    assert_eq!(second, 2);
}

#[test]
fn test_next_u64_large_value() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            self.0 += 1;
            self.0
        }
    }
    
    let mut rng = MockRng(u64::MAX - 1);
    let next = rng.next_u64();
    
    assert_eq!(next, u64::MAX);
}

#[test]
fn test_next_u64_zero_initialization() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            self.0 += 1;
            self.0
        }
    }
    
    let mut rng = MockRng(0);
    let zero_call = rng.next_u64();
    
    assert_eq!(zero_call, 1);
}

#[test]
#[should_panic]
fn test_next_u64_panic_on_overflow() {
    struct MockRng(u64);
    
    impl MockRng {
        fn next_u64(&mut self) -> u64 {
            self.0 += 1;
            if self.0 == 0 { // This will happen after u64::MAX
                panic!("Overflow detected");
            }
            self.0
        }
    }
    
    let mut rng = MockRng(u64::MAX);
    let _ = rng.next_u64(); // This should cause a panic on the next call
}

