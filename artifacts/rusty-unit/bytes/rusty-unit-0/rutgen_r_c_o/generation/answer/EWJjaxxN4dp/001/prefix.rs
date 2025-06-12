// Answer 0

#[test]
fn test_spare_capacity_mut_with_small_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 10); // Initial capacity must match
    
    uninit[0].write(0);
    unsafe { buf.set_len(1); }
    assert_eq!(buf.len(), 1);
}

#[test]
fn test_spare_capacity_mut_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(20);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 20); // Initial capacity must match
    
    for i in 0..19 {
        uninit[i].write(i as u8);
    }
    unsafe { buf.set_len(19); }
    assert_eq!(buf.len(), 19);
}

#[test]
fn test_spare_capacity_mut_with_full_initialization() {
    let mut buf = BytesMut::with_capacity(30);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 30);
    
    for i in 0..29 {
        uninit[i].write(i as u8);
    }
    unsafe { buf.set_len(29); }
    assert_eq!(buf.len(), 29);
}

#[test]
fn test_spare_capacity_mut_with_larger_cap() {
    let mut buf = BytesMut::with_capacity(64);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 64);
    
    for i in 0..63 {
        uninit[i].write(i as u8);
    }
    unsafe { buf.set_len(63); }
    assert_eq!(buf.len(), 63);
}

#[test]
#[should_panic]
fn test_spare_capacity_mut_with_excessive_write() {
    let mut buf = BytesMut::with_capacity(64);
    let uninit = buf.spare_capacity_mut();
    unsafe { buf.set_len(64); }
    
    // This write should panic since it goes over the available uninitialized space
    uninit[0].write(100);
}

#[test]
fn test_spare_capacity_mut_with_empty_buffer() {
    let mut buf = BytesMut::new();
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 0); // No capacity in new buffer
    
    buf.reserve(32);
    let uninit_after_reserve = buf.spare_capacity_mut();
    assert_eq!(uninit_after_reserve.len(), 32); // Should reflect the new reserved capacity
}

