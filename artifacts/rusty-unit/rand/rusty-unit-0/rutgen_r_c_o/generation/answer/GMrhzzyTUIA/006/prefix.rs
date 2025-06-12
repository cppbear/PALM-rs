// Answer 0

#[test]
fn test_fill_bytes_via_next_bound_n_0() {
    struct MockRng {}

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 0 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
    }

    let mut rng = MockRng {};
    let mut dest: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut dest);
}

#[test]
fn test_fill_bytes_via_next_bound_n_4() {
    struct MockRng {}

    impl RngCore for MockRng {
        fn next_u32(&mut self) -> u32 { 1 }
        fn next_u64(&mut self) -> u64 { 0 }
        fn fill_bytes(&mut self, dest: &mut [u8]) {
            for byte in dest.iter_mut() {
                *byte = 0;
            }
        }
    }

    let mut rng = MockRng {};
    let mut dest: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut dest);
}

