// Answer 0

#[test]
fn test_f64_from_parts_positive() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let result = deserializer.f64_from_parts(true, 123456789, 0).unwrap();
    assert_eq!(result, 123456789.0);
}

#[test]
fn test_f64_from_parts_negative() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let result = deserializer.f64_from_parts(false, 123456789, 0).unwrap();
    assert_eq!(result, -123456789.0);
}

#[test]
fn test_f64_from_parts_large_exponent() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let result = deserializer.f64_from_parts(true, 1, 300).unwrap();
    assert!(result.is_finite());
}

#[test]
fn test_f64_from_parts_infinite() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let result = deserializer.f64_from_parts(true, 1, 309);
    assert!(result.is_err());
}

#[test]
fn test_f64_from_parts_zero_significand() {
    let mut deserializer = Deserializer {
        read: SliceRead::new(&[]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let result = deserializer.f64_from_parts(true, 0, 10).unwrap();
    assert_eq!(result, 0.0);
}

