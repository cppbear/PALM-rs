// Answer 0

#[test]
fn test_put_bytes_with_enough_capacity() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 6][..];
    buf.put_bytes(b'a', 4);
    let result: Vec<u8> = buf.iter().map(|b| b.assume_init()).collect();
    assert_eq!(result, vec![b'a', b'a', b'a', b'a', 0, 0]);
}

#[test]
#[should_panic]
fn test_put_bytes_with_not_enough_capacity() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 2][..];
    buf.put_bytes(b'a', 4);
}

#[test]
fn test_put_bytes_fills_remaining_space() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 6][..];
    buf.put_bytes(b'a', 2);
    let result: Vec<u8> = buf.iter().map(|b| b.assume_init()).collect();
    assert_eq!(result, vec![b'a', b'a', 0, 0, 0, 0]);
}

#[test]
fn test_put_bytes_filling_exact_space() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 6][..];
    buf.put_bytes(b'a', 6);
    let result: Vec<u8> = buf.iter().map(|b| b.assume_init()).collect();
    assert_eq!(result, vec![b'a', b'a', b'a', b'a', b'a', b'a']);
}

#[test]
#[should_panic]
fn test_put_bytes_panic_on_zero_capacity() {
    let mut buf = &mut [core::mem::MaybeUninit::<u8>::uninit(); 0][..];
    buf.put_bytes(b'a', 1);
}

