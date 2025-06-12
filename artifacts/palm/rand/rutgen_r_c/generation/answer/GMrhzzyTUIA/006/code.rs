// Answer 0

#[derive(Default)]
struct MockRng {
    next_u64_counter: u64,
    next_u32_counter: u32,
}

impl rand_core::RngCore for MockRng {
    fn next_u32(&mut self) -> u32 {
        let value = self.next_u32_counter;
        self.next_u32_counter += 1;
        value
    }

    fn next_u64(&mut self) -> u64 {
        let value = self.next_u64_counter;
        self.next_u64_counter += 1;
        value
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.fill(0); // Not used in this test.
    }

    // Other methods from RngCore can be left unimplemented or default
}

#[test]
fn test_fill_bytes_via_next_less_than_8() {
    let mut rng = MockRng::default();
    let mut dest = [0u8; 7]; // testing with a slice length < 8
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [0, 0, 0, 0, 0, 0, 0]); // checks that the destination remains filled with 0
}

#[test]
fn test_fill_bytes_via_next_exactly_4() {
    let mut rng = MockRng::default();
    let mut dest = [0u8; 4]; // testing with a slice length of 4
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [0, 0, 0, 0]); // checks that the destination remains filled with 0
}

#[test]
fn test_fill_bytes_via_next_empty_slice() {
    let mut rng = MockRng::default();
    let mut dest: &mut [u8] = &mut []; // testing with an empty slice
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, []); // checks that the destination remains empty
}

