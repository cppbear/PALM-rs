// Answer 0

#[test]
fn test_advance_zero() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(0);
}

#[test]
fn test_advance_one() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(1);
}

#[test]
fn test_advance_two() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(2);
}

#[test]
fn test_advance_three() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(3);
}

#[test]
#[should_panic]
fn test_advance_four() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(4);
}

#[test]
#[should_panic]
fn test_advance_overflow() {
    let mut buf: &mut [u8] = &mut [1, 2, 3, 4];
    buf.advance(usize::MAX);
}

#[test]
fn test_advance_large_number() {
    let buf_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut buf: &mut [u8] = &mut buf_data;
    buf.advance(5);
}

