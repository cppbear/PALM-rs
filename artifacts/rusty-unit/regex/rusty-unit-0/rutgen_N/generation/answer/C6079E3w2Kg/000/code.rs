// Answer 0

#[test]
fn test_vb_with_valid_byte() {
    assert_eq!(vb(65), "A");
    assert_eq!(vb(10), "\n");
    assert_eq!(vb(32), " ");
    assert_eq!(vb(255), "\u{ff}");
}

#[test]
fn test_vb_with_out_of_range() {
    assert_eq!(vb(256), "EOF");
    assert_eq!(vb(300), "EOF");
}

#[test]
fn test_vb_with_zero() {
    assert_eq!(vb(0), "\0");
}

