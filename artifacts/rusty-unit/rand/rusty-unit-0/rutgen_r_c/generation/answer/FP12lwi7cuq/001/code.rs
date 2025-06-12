// Answer 0

#[test]
fn test_next_u32() {
    struct TestRng {
        value: u32,
    }

    impl RngCore for TestRng {
        fn next_u32(&mut self) -> u32 {
            self.value
        }
        
        fn next_u64(&mut self) -> u64 {
            0 // Not used in this test
        }

        fn fill_bytes(&mut self, _dst: &mut [u8]) {
            // Not used in this test
        }
    }

    let mut rng = StdRng(TestRng { value: 42 });

    assert_eq!(rng.next_u32(), 42);
    
    rng = StdRng(TestRng { value: 0 }); // Test lower boundary
    assert_eq!(rng.next_u32(), 0);

    rng = StdRng(TestRng { value: u32::MAX }); // Test upper boundary
    assert_eq!(rng.next_u32(), u32::MAX);
}

