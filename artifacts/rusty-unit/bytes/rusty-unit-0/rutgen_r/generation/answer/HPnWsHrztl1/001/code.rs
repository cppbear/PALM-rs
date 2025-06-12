// Answer 0

#[test]
fn test_without_provenance_zero() {
    let result = without_provenance(0);
    assert_eq!(result, core::ptr::null::<u8>());
}

#[test]
fn test_without_provenance_positive() {
    let ptr: usize = 5;
    let result = without_provenance(ptr);
    assert_eq!(result, (core::ptr::null::<u8>() as usize + ptr) as *const u8);
}

#[test]
fn test_without_provenance_large_value() {
    let ptr: usize = usize::MAX - 1; // testing near upper boundary
    let result = without_provenance(ptr);
    assert_eq!(result, (core::ptr::null::<u8>() as usize + ptr) as *const u8);
}

#[test]
fn test_without_provenance_large_value_plus_one() {
    let ptr: usize = usize::MAX; // testing upper boundary
    let result = without_provenance(ptr);
    assert_eq!(result, (core::ptr::null::<u8>() as usize + ptr) as *const u8);
}

