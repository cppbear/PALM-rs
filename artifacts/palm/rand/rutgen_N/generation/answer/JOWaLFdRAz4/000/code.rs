// Answer 0

#[derive(Default)]
struct MockRng;

impl MockRng {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = rand::random();
        }
    }
}

struct RngWrapper(MockRng);

impl RngWrapper {
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
    }
}

#[test]
fn test_fill_bytes_with_empty_slice() {
    let mut rng = RngWrapper(Default::default());
    let mut buffer: [u8; 0] = [];
    rng.fill_bytes(&mut buffer);
    assert_eq!(buffer.len(), 0);
}

#[test]
fn test_fill_bytes_with_small_slice() {
    let mut rng = RngWrapper(Default::default());
    let mut buffer = [0u8; 5];
    rng.fill_bytes(&mut buffer);
    assert_ne!(buffer, [0, 0, 0, 0, 0]);
}

#[test]
fn test_fill_bytes_with_large_slice() {
    let mut rng = RngWrapper(Default::default());
    let mut buffer = [0u8; 1024];
    rng.fill_bytes(&mut buffer);
    assert_ne!(buffer, [0; 1024]);
}

