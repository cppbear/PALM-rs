// Answer 0

#[test]
fn test_vb_zero() {
    vb(0);
}

#[test]
fn test_vb_min() {
    vb(1);
}

#[test]
fn test_vb_middle() {
    vb(127);
}

#[test]
fn test_vb_max() {
    vb(255);
}

#[test]
fn test_vb_above_max() {
    vb(256);
}

#[test]
fn test_vb_unicode() {
    vb(0xC3A9); // Example for utf-8 encoded é
}

