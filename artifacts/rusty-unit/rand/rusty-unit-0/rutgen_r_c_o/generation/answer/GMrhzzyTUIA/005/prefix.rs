// Answer 0

#[test]
fn test_fill_bytes_via_next_case_1() {
    struct MockRng {
        state: u64,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let result = self.state as u32;
            self.state = self.state.wrapping_add(1);
            result
        }

        fn next_u64(&mut self) -> u64 {
            let result = self.state;
            self.state = self.state.wrapping_add(1);
            result
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(0);
        }
        
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), std::io::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng { state: 1 };
    let mut dest = [0u8; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_case_2() {
    struct MockRng {
        state: u64,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let result = self.state as u32;
            self.state = self.state.wrapping_add(2);
            result
        }

        fn next_u64(&mut self) -> u64 {
            let result = self.state;
            self.state = self.state.wrapping_add(2);
            result
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(0);
        }
        
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), std::io::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng { state: 2 };
    let mut dest = [0u8; 3];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_case_3() {
    struct MockRng {
        state: u64,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let result = self.state as u32;
            self.state = self.state.wrapping_add(3);
            result
        }

        fn next_u64(&mut self) -> u64 {
            let result = self.state;
            self.state = self.state.wrapping_add(3);
            result
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            dest.fill(0);
        }
        
        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), std::io::Error> {
            self.fill_bytes(dest);
            Ok(())
        }
    }

    let mut rng = MockRng { state: 3 };
    let mut dest = [0u8; 1];
    fill_bytes_via_next(&mut rng, &mut dest);
}

