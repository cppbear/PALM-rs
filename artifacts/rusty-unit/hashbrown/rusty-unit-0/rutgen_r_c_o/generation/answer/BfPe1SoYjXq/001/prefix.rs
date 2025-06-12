// Answer 0

#[test]
fn test_raw_iter_range_new_valid_case() {
    let ctrl: *const u8 = &mut [0u8; 16] as *mut [u8; 16] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 4); // len is a power of two
    }
}

#[test]
fn test_raw_iter_range_new_power_of_two_boundary() {
    let ctrl: *const u8 = &mut [0u8; 8] as *mut [u8; 8] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 8); // len is max power of two within bounds
    }
}

#[test]
fn test_raw_iter_range_new_minimum_power_of_two() {
    let ctrl: *const u8 = &mut [0u8; 4] as *mut [u8; 4] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 1); // len is the minimum power of two
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_zero_length() {
    let ctrl: *const u8 = &mut [0u8; 16] as *mut [u8; 16] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 0); // len is zero, should panic
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_length_exceeds_buckets() {
    let ctrl: *const u8 = &mut [0u8; 16] as *mut [u8; 16] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 32); // len exceeds number of buckets, should panic
    }
}

#[test]
#[should_panic]
fn test_raw_iter_range_new_length_not_power_of_two() {
    let ctrl: *const u8 = &mut [0u8; 16] as *mut [u8; 16] as *const u8; // aligned to Group::WIDTH
    let bucket = Bucket {
        ptr: NonNull::new(ctrl as *mut u8).unwrap(),
    };
    unsafe {
        let _range = RawIterRange::new(ctrl, bucket, 3); // len is not a power of two, should panic
    }
}

