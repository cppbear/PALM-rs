// Answer 0

#[derive(Default)]
struct DummyRng {
    value: u64,
}

impl RngCore for DummyRng {
    fn next_u32(&mut self) -> u32 {
        self.value as u32
    }
    
    fn next_u64(&mut self) -> u64 {
        self.value
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.copy_from_slice(&self.next_u64().to_le_bytes());
    }
    
    fn jump(&mut self) {}

    fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}

#[test]
fn test_fill_bytes_via_next_empty_slice() {
    let mut rng = DummyRng::default();
    let mut dest = [];
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, []);
}

#[test]
fn test_fill_bytes_via_next_small_slice() {
    let mut rng = DummyRng::default();
    let mut dest = [0; 3];
    rng.set_value(0x0102030405060708);
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [0x08, 0x07, 0x06]);
}

#[test]
fn test_fill_bytes_via_next_exact_eight() {
    let mut rng = DummyRng::default();
    let mut dest = [0; 8];
    rng.set_value(0x0102030405060708);
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest, [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
}

#[test]
fn test_fill_bytes_via_next_large_slice() {
    let mut rng = DummyRng::default();
    let mut dest = [0; 16];
    rng.set_value(0x0102030405060708);
    fill_bytes_via_next(&mut rng, &mut dest);
    assert_eq!(dest[..8], [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
    assert_eq!(dest[8..], [0x08, 0x07, 0x06, 0x05]);
}

