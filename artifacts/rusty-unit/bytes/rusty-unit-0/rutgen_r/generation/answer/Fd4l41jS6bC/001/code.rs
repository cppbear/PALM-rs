// Answer 0

#[test]
fn test_uninit_valid_slice() {
    use bytes::buf::UninitSlice;
    use core::mem::MaybeUninit;

    let mut buffer = [MaybeUninit::uninit(); 64];
    let slice = UninitSlice::uninit(&mut buffer[..]);

    assert_eq!(slice.len(), 64);
}

#[test]
#[should_panic]
fn test_uninit_empty_slice() {
    use bytes::buf::UninitSlice;
    use core::mem::MaybeUninit;

    let mut buffer: [MaybeUninit<u8>; 0] = [];
    let _slice = UninitSlice::uninit(&mut buffer[..]);
}

#[test]
fn test_uninit_large_slice() {
    use bytes::buf::UninitSlice;
    use core::mem::MaybeUninit;

    let mut buffer = [MaybeUninit::uninit(); 1024];
    let slice = UninitSlice::uninit(&mut buffer[..]);

    assert_eq!(slice.len(), 1024);
}

#[test]
fn test_uninit_single_element() {
    use bytes::buf::UninitSlice;
    use core::mem::MaybeUninit;

    let mut buffer = [MaybeUninit::uninit(); 1];
    let slice = UninitSlice::uninit(&mut buffer[..]);

    assert_eq!(slice.len(), 1);
}

