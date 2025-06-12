// Answer 0

#[test]
#[should_panic]
fn test_new_uninitialized_buckets_zero() {
    let alloc = Global;
    let buckets = 0;
    let fallibility = Fallibility::Fallible;
    unsafe {
        RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
#[should_panic]
fn test_new_uninitialized_buckets_invalid_large() {
    let alloc = Global;
    let buckets = 3; // Not a power of two
    let fallibility = Fallibility::Fallible;
    unsafe {
        RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
#[should_panic]
fn test_new_uninitialized_buckets_exceeding_limit() {
    let alloc = Global;
    let buckets = 1073741825; // Exceeds maximum limit
    let fallibility = Fallibility::Fallible;
    unsafe {
        RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_minimum_buckets() {
    let alloc = Global;
    let buckets = 1; // Minimum power of two
    let fallibility = Fallibility::Infallible;
    unsafe {
        let _ = RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_small_power_of_two() {
    let alloc = Global;
    let buckets = 4; // Valid power of two
    let fallibility = Fallibility::Infallible;
    unsafe {
        let _ = RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_large_power_of_two() {
    let alloc = Global;
    let buckets = 1024; // Larger valid power of two
    let fallibility = Fallibility::Infallible;
    unsafe {
        let _ = RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

#[test]
fn test_new_uninitialized_max_power_of_two() {
    let alloc = Global;
    let buckets = 1073741824; // Maximum valid power of two
    let fallibility = Fallibility::Infallible;
    unsafe {
        let _ = RawTable::<u8, Global>::new_uninitialized(alloc, buckets, fallibility);
    }
}

