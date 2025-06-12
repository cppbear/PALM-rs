// Answer 0

#[test]
fn test_zeroed_with_zero_length() {
    let _ = zeroed(0);
}

#[test]
fn test_zeroed_with_small_length() {
    let _ = zeroed(5);
}

#[test]
fn test_zeroed_with_large_length() {
    let _ = zeroed(1024);
}

#[test]
fn test_zeroed_with_max_length() {
    let _ = zeroed(usize::MAX);
}

