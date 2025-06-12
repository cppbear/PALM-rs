// Answer 0

#[test]
fn test_spare_capacity_mut_initial_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    let spare_capacity = buf.spare_capacity_mut();
    assert_eq!(spare_capacity.len(), 10);
}

#[test]
fn test_spare_capacity_mut_after_initialization() {
    let mut buf = BytesMut::with_capacity(10);
    let spare_capacity = buf.spare_capacity_mut();
    assert_eq!(spare_capacity.len(), 10);
    unsafe {
        spare_capacity[0].write(1);
        spare_capacity[1].write(2);
    }
    buf.set_len(2);
    assert_eq!(&buf[..], &[1, 2]);
}

#[test]
fn test_spare_capacity_mut_beyond_initialization() {
    let mut buf = BytesMut::with_capacity(10);
    let spare_capacity = buf.spare_capacity_mut();
    unsafe {
        spare_capacity[0].write(1);
        buf.set_len(1);
    }
    let new_spare_capacity = buf.spare_capacity_mut();
    assert_eq!(new_spare_capacity.len(), 9);
}

#[test]
fn test_spare_capacity_mut_zero_capacity() {
    let mut buf = BytesMut::new();
    let spare_capacity = buf.spare_capacity_mut();
    assert_eq!(spare_capacity.len(), 0);
}

