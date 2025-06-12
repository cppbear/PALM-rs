// Answer 0

#[test]
fn advance_mut_test_valid_case() {
    let mut buffer: [core::mem::MaybeUninit<u8>; 10] = Default::default();
    let mut buf_mut = &mut buffer[..];

    unsafe { buf_mut.advance_mut(5); }

    assert_eq!(buf_mut.len(), 5);
}

#[test]
#[should_panic(expected = "TryGetError")]
fn advance_mut_test_exceed_length() {
    let mut buffer: [core::mem::MaybeUninit<u8>; 5] = Default::default();
    let mut buf_mut = &mut buffer[..];

    unsafe { buf_mut.advance_mut(6); }
}

#[test]
fn advance_mut_test_edge_case_zero_length() {
    let mut buffer: [core::mem::MaybeUninit<u8>; 0] = Default::default();
    let mut buf_mut = &mut buffer[..];

    unsafe { buf_mut.advance_mut(0); }

    assert_eq!(buf_mut.len(), 0);
}

#[test]
fn advance_mut_test_non_zero_memory() {
    let mut buffer: [core::mem::MaybeUninit<u8>; 3] = Default::default();
    let mut buf_mut = &mut buffer[..];

    unsafe { buf_mut.advance_mut(2); }

    assert_eq!(buf_mut.len(), 1);  // 1 should remain after advancing by 2
}

#[test]
#[should_panic(expected = "TryGetError")]
fn advance_mut_test_panic_on_same_length() {
    let mut buffer: [core::mem::MaybeUninit<u8>; 2] = Default::default();
    let mut buf_mut = &mut buffer[..];

    unsafe { buf_mut.advance_mut(2); }
}

