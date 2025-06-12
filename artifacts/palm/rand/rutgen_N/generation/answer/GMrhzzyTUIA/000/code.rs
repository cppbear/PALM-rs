// Answer 0

#[derive(Default)]
struct TestRng {
    value: u64,
}

impl TestRng {
    fn new() -> Self {
        Self { value: 0 }
    }
}

impl rand_core::RngCore for TestRng {
    fn next_u32(&mut self) -> u32 {
        self.value += 1;
        self.value as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.value += 1;
        self.value
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let _ = dest;
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[test]
fn test_fill_bytes_via_next_full_segments() {
    let mut rng = TestRng::new();
    let mut buffer = [0u8; 16];
    fill_bytes_via_next(&mut rng, &mut buffer);
    assert_eq!(buffer, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_via_next_partial_segment() {
    let mut rng = TestRng::new();
    let mut buffer = [0u8; 10];
    fill_bytes_via_next(&mut rng, &mut buffer);
    assert_eq!(buffer, [1, 0, 0, 0, 2, 0, 0, 0, 3, 0]);
}

#[test]
fn test_fill_bytes_via_next_small_buffer() {
    let mut rng = TestRng::new();
    let mut buffer = [0u8; 4];
    fill_bytes_via_next(&mut rng, &mut buffer);
    assert_eq!(buffer, [1, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_via_next_empty_buffer() {
    let mut rng = TestRng::new();
    let mut buffer: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut buffer);
    assert_eq!(buffer, []);
}

