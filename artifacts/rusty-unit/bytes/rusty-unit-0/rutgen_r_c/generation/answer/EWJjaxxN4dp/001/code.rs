// Answer 0

#[test]
fn test_spare_capacity_mut_initialization() {
    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 10); // Should have full capacity available
}

#[test]
fn test_spare_capacity_mut_after_setting_length() {
    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();
    unsafe {
        uninit[0].write(0);
        uninit[1].write(1);
        uninit[2].write(2);
    }
    buf.set_len(3); // Set length to 3
    let uninit_after = buf.spare_capacity_mut();
    assert_eq!(uninit_after.len(), 7); // Should have 7 bytes spare capacity after setting length
}

#[test]
fn test_spare_capacity_mut_zero_length() {
    let mut buf = BytesMut::new();
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 0); // Should have no spare capacity
}

#[test]
fn test_spare_capacity_mut_boundary_condition() {
    let mut buf = BytesMut::with_capacity(1);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 1); // Should have 1 spare capacity
    unsafe {
        uninit[0].write(255);
    }
    buf.set_len(1);
    let uninit_after = buf.spare_capacity_mut();
    assert_eq!(uninit_after.len(), 0); // Should have 0 spare capacity after setting length
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_spare_capacity_mut_access_out_of_bounds() {
    let mut buf = BytesMut::with_capacity(1);
    let uninit = buf.spare_capacity_mut();
    unsafe {
        uninit[0].write(0);
    }
    buf.set_len(1);
    let uninit_after = buf.spare_capacity_mut();
    // Accessing index 1 should panic
    let _ = unsafe { uninit_after[1] };
}

