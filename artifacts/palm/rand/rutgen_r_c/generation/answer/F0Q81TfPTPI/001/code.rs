// Answer 0

#[test]
fn test_flip_c_heads_boundaries() {
    struct MockRng {
        current: u32,
        counter: usize,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.current = if self.counter < 1 {
                0b11111111111111111111111111111111 // 32 zeros
            } else {
                0b00000000000000000000000000001111 // 4 ones
            };
            self.counter += 1;
            self.current
        }
        
        fn next_u64(&mut self) -> u64 {
            unimplemented!()
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            unimplemented!()
        }
        
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!()
        }
    }

    let mut rng = MockRng { current: 0, counter: 0 };
    let mut flipper = CoinFlipper::new(rng);

    let result = flipper.flip_c_heads(32);
    assert_eq!(result, false); // Should return false as 32 bits are all heads
}

#[test]
fn test_flip_c_heads_zeros_less_than_c() {
    struct MockRng {
        current: u32,
        counter: usize,
    }
    
    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            self.current = 0b11000000000000000000000000000000; // 2 heads followed by 30 tails
            self.counter += 1;
            self.current
        }
        
        fn next_u64(&mut self) -> u64 {
            unimplemented!()
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            unimplemented!()
        }
        
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!()
        }
    }

    let mut rng = MockRng { current: 0, counter: 0 };
    let mut flipper = CoinFlipper::new(rng);

    let result = flipper.flip_c_heads(2);
    assert_eq!(result, false); // Should return false as the first 2 bits were heads
}

