// Answer 0

#[test]
fn test_advance_mut_zero_cnt() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(0); }
    unsafe { bytes_mut.advance_mut(0); }
}

#[test]
fn test_advance_mut_full_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5); }
    unsafe { bytes_mut.advance_mut(5); }
}

#[test]
fn test_advance_mut_partial_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5); }
    unsafe { bytes_mut.advance_mut(3); }
}

#[test]
fn test_advance_mut_multiple_zero() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    unsafe { bytes_mut.set_len(0); }
    unsafe { bytes_mut.advance_mut(0); }
}

#[test]
#[should_panic]
fn test_advance_mut_panic_exceeding_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5); }
    unsafe { bytes_mut.advance_mut(6); }
}

