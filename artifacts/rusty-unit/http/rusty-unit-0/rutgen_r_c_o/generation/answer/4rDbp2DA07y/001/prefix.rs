// Answer 0

#[test]
fn test_status_code_as_u16_100() {
    let status = StatusCode(NonZeroU16::new(100).unwrap());
    let _ = status.as_u16();
}

#[test]
fn test_status_code_as_u16_200() {
    let status = StatusCode(NonZeroU16::new(200).unwrap());
    let _ = status.as_u16();
}

#[test]
fn test_status_code_as_u16_300() {
    let status = StatusCode(NonZeroU16::new(300).unwrap());
    let _ = status.as_u16();
}

#[test]
fn test_status_code_as_u16_400() {
    let status = StatusCode(NonZeroU16::new(400).unwrap());
    let _ = status.as_u16();
}

#[test]
fn test_status_code_as_u16_500() {
    let status = StatusCode(NonZeroU16::new(500).unwrap());
    let _ = status.as_u16();
}

#[test]
fn test_status_code_as_u16_511() {
    let status = StatusCode(NonZeroU16::new(511).unwrap());
    let _ = status.as_u16();
}

