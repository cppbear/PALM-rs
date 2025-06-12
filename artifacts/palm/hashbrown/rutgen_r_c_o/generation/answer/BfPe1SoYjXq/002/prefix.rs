// Answer 0

#[test]
fn test_raw_iter_range_new_valid_case() {
    let len = 4; // A power of two
    let ctrl: *const u8 = 0x1000 as *const u8; // Properly aligned
    let bucket = Bucket { ptr: NonNull::new(0x2000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_invalid_ctrl_alignment() {
    let len = 4; // A power of two
    let ctrl: *const u8 = 0x1001 as *const u8; // Misaligned
    let bucket = Bucket { ptr: NonNull::new(0x2000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_zero_length() {
    let len = 0; // Zero length
    let ctrl: *const u8 = 0x1000 as *const u8; // Properly aligned
    let bucket = Bucket { ptr: NonNull::new(0x2000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_length_not_power_of_two() {
    let len = 3; // Not a power of two
    let ctrl: *const u8 = 0x1000 as *const u8; // Properly aligned
    let bucket = Bucket { ptr: NonNull::new(0x2000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_out_of_bounds_ctrl() {
    let len = 4; // A power of two
    let ctrl: *const u8 = 0xFFFFFFFF as *const u8; // Out of bounds
    let bucket = Bucket { ptr: NonNull::new(0x2000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

#[test]
fn test_raw_iter_range_new_valid_case_noncontrol_bucket() {
    let len = 8; // A power of two
    let ctrl: *const u8 = 0x2000 as *const u8; // Properly aligned
    let bucket = Bucket { ptr: NonNull::new(0x3000 as *mut u8).unwrap() }; // Valid bucket

    unsafe {
        let iter_range = RawIterRange::new(ctrl, bucket, len);
    }
}

