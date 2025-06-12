// Answer 0

#[test]
fn test_advance_with_sufficient_data() {
    let mut buf: &[u8] = &mut [1, 2, 3, 4, 5];
    buf.advance(2);
    assert_eq!(buf, &mut [3, 4, 5]);
}

#[test]
#[should_panic(expected = "TryGetError { requested: 4, available: 2 }")]
fn test_advance_with_insufficient_data() {
    let mut buf: &mut [u8] = &mut [1, 2];
    buf.advance(4);
}

#[test]
fn test_advance_boundary_case() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4, 5];
    buf.advance(5);
    assert_eq!(buf, &mut []);
}

#[test]
fn test_advance_on_empty_buffer() {
    let mut buf: &mut [u8] = &mut [];
    buf.advance(0);
    assert_eq!(buf, &mut []);
}

