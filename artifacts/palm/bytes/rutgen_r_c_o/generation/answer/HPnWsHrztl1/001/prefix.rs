// Answer 0

#[test]
fn test_without_provenance_zero() {
    let ptr: usize = 0;
    let result = without_provenance(ptr);
}

#[test]
fn test_without_provenance_small_value() {
    let ptr: usize = 1;
    let result = without_provenance(ptr);
}

#[test]
fn test_without_provenance_mid_value() {
    let ptr: usize = usize::MAX / 2;
    let result = without_provenance(ptr);
}

#[test]
fn test_without_provenance_large_value() {
    let ptr: usize = usize::MAX - 1;
    let result = without_provenance(ptr);
}

#[test]
fn test_without_provenance_max_value() {
    let ptr: usize = usize::MAX;
    let result = without_provenance(ptr);
}

