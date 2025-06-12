// Answer 0

#[test]
fn test_advance_mut_panic_condition_1() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5) };
    unsafe { bytes_mut.advance_mut(6) };
}

#[test]
fn test_advance_mut_panic_condition_2() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe { bytes_mut.set_len(15) };
    unsafe { bytes_mut.advance_mut(16) };
}

#[test]
fn test_advance_mut_panic_condition_large_count() {
    let mut bytes_mut = BytesMut::with_capacity(100);
    unsafe { bytes_mut.set_len(90) };
    unsafe { bytes_mut.advance_mut(101) };
}

#[test]
fn test_advance_mut_panic_condition_edge_case() {
    let mut bytes_mut = BytesMut::with_capacity(0);
    unsafe { bytes_mut.set_len(0) };
    unsafe { bytes_mut.advance_mut(1) };
}

