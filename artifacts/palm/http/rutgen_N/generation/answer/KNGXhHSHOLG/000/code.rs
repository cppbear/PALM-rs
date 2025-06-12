// Answer 0

#[derive(Default)]
struct U64Wrapper(u64);

impl U64Wrapper {
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }
}

#[test]
fn test_write_u64_updates_value() {
    let mut wrapper = U64Wrapper::default();
    wrapper.write_u64(42);
    assert_eq!(wrapper.0, 42);
}

#[test]
fn test_write_u64_edge_case_zero() {
    let mut wrapper = U64Wrapper::default();
    wrapper.write_u64(0);
    assert_eq!(wrapper.0, 0);
}

#[test]
fn test_write_u64_large_value() {
    let mut wrapper = U64Wrapper::default();
    wrapper.write_u64(u64::MAX);
    assert_eq!(wrapper.0, u64::MAX);
}

