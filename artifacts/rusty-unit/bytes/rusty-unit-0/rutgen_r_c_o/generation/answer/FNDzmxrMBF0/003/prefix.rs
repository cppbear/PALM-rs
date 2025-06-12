// Answer 0

#[test]
fn test_put_slice_exact_fit() {
    let mut dst: [core::mem::MaybeUninit<u8>; 5] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(b"hello");
}

#[test]
fn test_put_slice_partially_filled() {
    let mut dst: [core::mem::MaybeUninit<u8>; 10] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(b"hello");
    buf.put_slice(b"!");
}

#[test]
fn test_put_slice_multiple_chunks() {
    let mut dst: [core::mem::MaybeUninit<u8>; 10] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(b"hello");
    buf.put_slice(b" world");
}

#[test]
#[should_panic]
fn test_put_slice_too_large() {
    let mut dst: [core::mem::MaybeUninit<u8>; 4] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(b"hello");
}

#[test]
fn test_put_slice_empty_src() {
    let mut dst: [core::mem::MaybeUninit<u8>; 5] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(&[] as &[u8]);
}

#[test]
fn test_put_slice_reaching_zero_remaining() {
    let mut dst: [core::mem::MaybeUninit<u8>; 6] = Default::default();
    let mut buf = &mut dst as &mut [core::mem::MaybeUninit<u8>];

    buf.put_slice(b"hello");
    buf.put_slice(&[] as &[u8]);
}

