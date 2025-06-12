// Answer 0

#[test]
fn test_fill_u16() {
    struct MockRng {
        data: Vec<u16>,
        current: usize,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.data[self.current] as u32;
            self.current += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            let value = self.data[self.current] as u64;
            self.current += 1;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let bytes = &self.data[self.current..self.current + (dest.len() / 2)];
            for (chunk, byte) in dest.chunks_mut(2).zip(bytes) {
                chunk.copy_from_slice(&byte.to_le_bytes());
            }
            self.current += dest.len() / 2;
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ()> {
            self.fill_bytes(dest);
            Ok(())
        }

        fn fill(&mut self, dest: &mut [u8]) {
            self.fill_bytes(dest);
        }
    }

    let mut rng = MockRng {
        data: vec![1, 2, 3, 4, 5],
        current: 0,
    };
    
    let mut array: &mut [u16] = &mut [0; 5];
    array.fill(&mut rng);
    assert_eq!(array, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_fill_i32() {
    struct MockRng {
        data: Vec<i32>,
        current: usize,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.data[self.current] as u32;
            self.current += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            let value = self.data[self.current] as u64;
            self.current += 1;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            let bytes = &self.data[self.current..self.current + (dest.len() / 4)];
            for (chunk, byte) in dest.chunks_mut(4).zip(bytes) {
                chunk.copy_from_slice(&byte.to_le_bytes());
            }
            self.current += dest.len() / 4;
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ()> {
            self.fill_bytes(dest);
            Ok(())
        }

        fn fill(&mut self, dest: &mut [u8]) {
            self.fill_bytes(dest);
        }
    }

    let mut rng = MockRng {
        data: vec![-1, -2, -3, -4, -5],
        current: 0,
    };

    let mut array: &mut [i32] = &mut [0; 5];
    array.fill(&mut rng);
    assert_eq!(array, &[-1, -2, -3, -4, -5]);
}

#[test]
fn test_fill_bool() {
    struct MockRng {
        data: Vec<bool>,
        current: usize,
    }

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.data[self.current] as u32;
            self.current += 1;
            value
        }

        fn next_u64(&mut self) -> u64 {
            let value = self.data[self.current] as u64;
            self.current += 1;
            value
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = if self.data[self.current] { 1 } else { 0 };
                self.current += 1;
            }
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ()> {
            self.fill_bytes(dest);
            Ok(())
        }

        fn fill(&mut self, dest: &mut [u8]) {
            self.fill_bytes(dest);
        }
    }

    let mut rng = MockRng {
        data: vec![true, false, true, true, false],
        current: 0,
    };

    let mut array: &mut [bool] = &mut [false; 5];
    array.fill(&mut rng);
    assert_eq!(array, &[true, false, true, true, false]);
}

