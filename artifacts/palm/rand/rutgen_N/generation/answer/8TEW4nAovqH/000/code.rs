// Answer 0

#[derive(Debug)]
struct MyType(u32);

impl MyType {
    fn to_le_bytes(self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

#[test]
fn test_to_le_bytes() {
    let value = MyType(1);
    let bytes = value.to_le_bytes();
    assert_eq!(bytes, [1, 0, 0, 0]);
}

#[test]
fn test_to_le_bytes_zero() {
    let value = MyType(0);
    let bytes = value.to_le_bytes();
    assert_eq!(bytes, [0, 0, 0, 0]);
}

#[test]
fn test_to_le_bytes_max() {
    let value = MyType(u32::MAX);
    let bytes = value.to_le_bytes();
    assert_eq!(bytes, [255, 255, 255, 255]);
}

