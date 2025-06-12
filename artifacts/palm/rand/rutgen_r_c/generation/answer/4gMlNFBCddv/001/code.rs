// Answer 0

#[test]
fn test_append_string_empty() {
    use rand::rngs::MockRng;

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn gen_u32(&mut self) -> u32 {
            let result = self.current as u32 % 26 + b'a' as u32;
            self.current += 1;
            result
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.gen_u32() as u8;
            }
        }
    }

    let mut rng = MockRng { current: 0 };
    let alphabetic = Alphabetic;
    let mut output = String::new();
    alphabetic.append_string(&mut rng, &mut output, 0);
    assert_eq!(output.len(), 0);
}

#[test]
fn test_append_string_non_empty() {
    use rand::rngs::MockRng;

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn gen_u32(&mut self) -> u32 {
            let result = self.current as u32 % 26 + b'a' as u32;
            self.current += 1;
            result
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.gen_u32() as u8;
            }
        }
    }

    let mut rng = MockRng { current: 0 };
    let alphabetic = Alphabetic;
    let mut output = String::new();
    alphabetic.append_string(&mut rng, &mut output, 5);
    assert_eq!(output.len(), 5);
    assert!(output.chars().all(|c| c.is_ascii_lowercase()));
}

#[test]
#[should_panic]
fn test_append_string_exceed_capacity() {
    use rand::rngs::MockRng;

    struct MockRng {
        current: usize,
    }

    impl Rng for MockRng {
        fn gen_u32(&mut self) -> u32 {
            let result = self.current as u32 % 26 + b'a' as u32;
            self.current += 1;
            result
        }
        
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest {
                *byte = self.gen_u32() as u8;
            }
        }
    }

    let mut rng = MockRng { current: 0 };
    let alphabetic = Alphabetic;
    let mut output = String::with_capacity(3);
    alphabetic.append_string(&mut rng, &mut output, 4); // This should panic due to exceeding capacity
}

