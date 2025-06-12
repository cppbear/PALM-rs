// Answer 0

#[test]
fn test_spare_capacity_mut() {
    use bytes::BytesMut;
    use std::mem::MaybeUninit;

    // Allocate buffer big enough for 10 bytes.
    let mut buf = BytesMut::with_capacity(10);

    // Fill in the first 3 elements.
    let uninit = buf.spare_capacity_mut();
    uninit[0].write(0);
    uninit[1].write(1);
    uninit[2].write(2);

    // Mark the first 3 bytes of the buffer as being initialized.
    unsafe {
        buf.set_len(3);
    }

    assert_eq!(&buf[..], &[0, 1, 2]);
}

#[test]
fn test_spare_capacity_mut_empty() {
    use bytes::BytesMut;
    use std::mem::MaybeUninit;

    // Allocate buffer with zero capacity.
    let mut buf = BytesMut::with_capacity(0);

    // Get spare capacity (should be empty).
    let uninit = buf.spare_capacity_mut();
    
    // No uninitialized space to write into.
    assert!(uninit.is_empty());
}

#[test]
fn test_spare_capacity_mut_boundary() {
    use bytes::BytesMut;
    use std::mem::MaybeUninit;

    // Allocate buffer big enough for 5 bytes.
    let mut buf = BytesMut::with_capacity(5);

    // Fill the buffer completely.
    let uninit = buf.spare_capacity_mut();
    for i in 0..5 {
        uninit[i].write(i as u8);
    }

    // Mark all 5 bytes of the buffer as being initialized.
    unsafe {
        buf.set_len(5);
    }

    assert_eq!(&buf[..], &[0, 1, 2, 3, 4]);
}

