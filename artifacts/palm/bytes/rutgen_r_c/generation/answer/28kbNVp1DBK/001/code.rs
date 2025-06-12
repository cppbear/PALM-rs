// Answer 0

#[test]
fn test_remaining_mut() {
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 10];
    assert_eq!(buf.remaining_mut(), 10);
}

#[test]
fn test_remaining_mut_empty() {
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut [];
    assert_eq!(buf.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_non_empty() {
    let buf: &mut [core::mem::MaybeUninit<u8>] = &mut [core::mem::MaybeUninit::uninit(); 5];
    assert_eq!(buf.remaining_mut(), 5);
}

