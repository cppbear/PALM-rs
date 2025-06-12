// Answer 0

#[test]
fn test_without_provenance_zero() {
    let ptr = 0;
    let result = without_provenance(ptr);
    assert_eq!(result, core::ptr::null::<u8>());
}

#[test]
fn test_without_provenance_positive() {
    let ptr = 10;
    let result = without_provenance(ptr);
    assert_eq!(result as usize, ptr);
}

#[test]
fn test_without_provenance_large_value() {
    let ptr = usize::MAX;
    let result = without_provenance(ptr);
    assert_eq!(result as usize, ptr);
}

#[should_panic]
fn test_without_provenance_negative() {
    let ptr = -(1 as usize); // This will not actually compile as `usize` cannot be negative
    let _result = without_provenance(ptr);
}

