// Answer 0

#[test]
fn test_without_provenance_zero() {
    let result = without_provenance(0);
    assert_eq!(result, core::ptr::null::<u8>());
}

#[test]
fn test_without_provenance_non_zero() {
    let ptr_value: usize = 10;
    let result = without_provenance(ptr_value);
    let expected = core::ptr::null::<u8>().wrapping_add(ptr_value);
    assert_eq!(result, expected);
}

#[test]
fn test_without_provenance_boundary() {
    let max_size: usize = usize::MAX;
    let result = without_provenance(max_size);
    let expected = core::ptr::null::<u8>().wrapping_add(max_size);
    assert_eq!(result, expected);
}

