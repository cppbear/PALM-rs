// Answer 0

#[test]
fn test_spare_capacity_mut_with_full_capacity() {
    use std::mem::MaybeUninit;
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();
    assert_eq!(uninit.len(), 10); // Capacity equals spare capacity initially
}

#[test]
fn test_spare_capacity_mut_with_partial_initialization() {
    use std::mem::MaybeUninit;
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();

    uninit[0].write(0);
    uninit[1].write(1);
    uninit[2].write(2);

    unsafe {
        buf.set_len(3);
    }

    let uninit_after = buf.spare_capacity_mut();
    assert_eq!(uninit_after.len(), 7);  // Remaining spare capacity after writing 3 bytes
}

#[test]
fn test_spare_capacity_mut_with_all_initialization() {
    use std::mem::MaybeUninit;
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(5);
    let uninit = buf.spare_capacity_mut();

    uninit[0].write(0);
    uninit[1].write(1);
    uninit[2].write(2);
    uninit[3].write(3);
    uninit[4].write(4);

    unsafe {
        buf.set_len(5); // Setting length equal to capacity
    }

    let uninit_after = buf.spare_capacity_mut();
    assert_eq!(uninit_after.len(), 0); // No spare capacity left
}

#[should_panic]
fn test_spare_capacity_mut_with_overflow_write() {
    use std::mem::MaybeUninit;
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(10);
    let uninit = buf.spare_capacity_mut();

    uninit[10].write(0); // This should panic as we are writing outside allocated space
}

#[test]
fn test_spare_capacity_mut_after_setting_length() {
    use std::mem::MaybeUninit;
    use bytes::BytesMut;

    let mut buf = BytesMut::with_capacity(15);
    let uninit = buf.spare_capacity_mut();

    uninit[0].write(0);
    uninit[1].write(1);

    unsafe {
        buf.set_len(2); // Mark first 2 bytes as initialized
    }

    let uninit_after = buf.spare_capacity_mut();
    assert_eq!(uninit_after.len(), 13); // Check spare capacity after marking length
}

