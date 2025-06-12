// Answer 0

#[test]
#[should_panic]
fn test_abort_std_feature() {
    // This test will trigger the abort function under the std feature.
    let _ = abort();
}

#[test]
#[should_panic]
fn test_abort_not_std_feature() {
    // This test will trigger the abort function under the not std feature.
    // Note: In order to execute this test, the "std" feature must be disabled.
    let _ = abort();
}

