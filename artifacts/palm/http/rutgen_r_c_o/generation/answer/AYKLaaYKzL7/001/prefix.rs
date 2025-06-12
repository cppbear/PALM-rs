// Answer 0

#[test]
fn test_status_with_negative_value() {
    let builder = Builder::new();
    builder.status(-1);
}

#[test]
fn test_status_with_zero() {
    let builder = Builder::new();
    builder.status(0);
}

#[test]
fn test_status_with_one() {
    let builder = Builder::new();
    builder.status(1);
}

#[test]
fn test_status_with_large_value() {
    let builder = Builder::new();
    builder.status(999);
}

#[test]
fn test_status_with_max_value() {
    let builder = Builder::new();
    builder.status(1000);
}

#[test]
fn test_status_with_valid_conversion() {
    let builder = Builder::new();
    builder.status(200);
}

