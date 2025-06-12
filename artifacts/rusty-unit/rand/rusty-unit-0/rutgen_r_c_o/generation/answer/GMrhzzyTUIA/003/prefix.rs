// Answer 0

#[derive(Default)]
struct MockRng {
    value: u64,
}

impl RngCore for MockRng {
    fn next_u32(&mut self) -> u32 {
        (self.value & 0xFFFFFFFF) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.value
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.value.to_le_bytes());
    }

    fn jumpable(&self) -> bool {
        false
    }
}

#[test]
fn test_fill_bytes_via_next_edge_case_8() {
    let mut rng = MockRng { value: 123456789 };
    let mut buffer: [u8; 8] = [0; 8];
    fill_bytes_via_next(&mut rng, &mut buffer);
}

#[test]
fn test_fill_bytes_via_next_edge_case_4() {
    let mut rng = MockRng { value: 123456789 };
    let mut buffer: [u8; 4] = [0; 4];
    fill_bytes_via_next(&mut rng, &mut buffer);
}

#[test]
fn test_fill_bytes_via_next_edge_case_0() {
    let mut rng = MockRng { value: 123456789 };
    let mut buffer: [u8; 0] = [];
    fill_bytes_via_next(&mut rng, &mut buffer);
}

