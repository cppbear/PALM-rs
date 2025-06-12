// Answer 0

#[test]
fn test_append_string_empty() {
    use rand::RngCore;
    use rand::rngs::OsRng;

    struct DummyRng {
        idx: usize,
        data: Vec<u8>,
    }

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.next_u8();
            }
        }

        fn random_u32(&mut self) -> u32 {
            self.next_u32()
        }

        fn random_u64(&mut self) -> u64 {
            self.next_u64()
        }
    }

    let mut rng = DummyRng { idx: 0, data: vec![b'A', b'1'] };
    let alphanumeric = Alphanumeric;
    let mut result = String::new();

    alphanumeric.append_string(&mut rng, &mut result, 0);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_append_string_non_empty() {
    use rand::RngCore;
    use rand::rngs::OsRng;

    struct DummyRng {
        idx: usize,
        data: Vec<u8>,
    }

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.next_u8();
            }
        }

        fn random_u32(&mut self) -> u32 {
            self.next_u32()
        }

        fn random_u64(&mut self) -> u64 {
            self.next_u64()
        }
    }

    let mut rng = DummyRng { idx: 0, data: vec![b'A', b'1'] };
    let alphanumeric = Alphanumeric;
    let mut result = String::new();

    alphanumeric.append_string(&mut rng, &mut result, 10);
    assert_eq!(result.len(), 10);
    assert!(result.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
#[should_panic]
fn test_append_string_negative_length() {
    use rand::RngCore;
    use rand::rngs::OsRng;

    struct DummyRng {
        idx: usize,
        data: Vec<u8>,
    }

    impl RngCore for DummyRng {
        fn next_u32(&mut self) -> u32 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u32
        }

        fn next_u64(&mut self) -> u64 {
            self.idx += 1;
            self.data[self.idx % self.data.len()] as u64
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = self.next_u8();
            }
        }

        fn random_u32(&mut self) -> u32 {
            self.next_u32()
        }

        fn random_u64(&mut self) -> u64 {
            self.next_u64()
        }
    }

    let mut rng = DummyRng { idx: 0, data: vec![b'A', b'1'] };
    let alphanumeric = Alphanumeric;
    let mut result = String::new();

    alphanumeric.append_string(&mut rng, &mut result, usize::MAX);
}

