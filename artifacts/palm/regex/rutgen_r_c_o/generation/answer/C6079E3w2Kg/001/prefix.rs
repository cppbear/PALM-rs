// Answer 0

#[test]
fn test_vb_above_u8_max() {
    let _ = vb(256);
}

#[test]
fn test_vb_generic_large_value() {
    let _ = vb(1024);
}

#[test]
fn test_vb_maximum_value() {
    let _ = vb(4294967295);
}

#[test]
fn test_vb_small_window() {
    let _ = vb(300);
}

