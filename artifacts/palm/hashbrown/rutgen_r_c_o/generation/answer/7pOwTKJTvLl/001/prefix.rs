// Answer 0

#[test]
fn test_read_nonzero_valid_index() {
    let base: NonNull<i32> = NonNull::new(Box::into_raw(Box::new(42))).unwrap();
    let bucket = unsafe { Bucket::<i32>::from_base_index(base, 1) };
    unsafe { bucket.write(17) };
    let value = unsafe { bucket.read() };
}

#[test]
fn test_read_nonzero_edge_case() {
    let base: NonNull<i32> = NonNull::new(Box::into_raw(Box::new(100))).unwrap();
    let bucket = unsafe { Bucket::<i32>::from_base_index(base, 0) };
    unsafe { bucket.write(99) };
    let value = unsafe { bucket.read() };
}

#[test]
#[should_panic]
fn test_read_zero_sized() {
    struct ZeroSized;
    let base: NonNull<ZeroSized> = NonNull::new(Box::into_raw(Box::new(ZeroSized))).unwrap();
    let bucket = unsafe { Bucket::<ZeroSized>::from_base_index(base, 1) };
    let _value = unsafe { bucket.read() }; // This should panic
}

#[test]
fn test_read_nonzero_large_index() {
    let base: NonNull<i32> = NonNull::new(Box::into_raw(Box::new(25))).unwrap();
    let bucket = unsafe { Bucket::<i32>::from_base_index(base, 500) };
    unsafe { bucket.write(64) };
    let value = unsafe { bucket.read() };
}

#[test]
fn test_read_nonzero_multiple_entries() {
    let base: NonNull<i32> = NonNull::new(Box::into_raw(Box::new(1))).unwrap();
    let bucket1 = unsafe { Bucket::<i32>::from_base_index(base, 2) };
    let bucket2 = unsafe { Bucket::<i32>::from_base_index(base, 3) };
    unsafe { bucket1.write(10) };
    unsafe { bucket2.write(20) };
    let value1 = unsafe { bucket1.read() };
    let value2 = unsafe { bucket2.read() };
}

