// Answer 0

#[test]
fn test_uninit_u8_array_returns_correct_size() {
    let result = uninit_u8_array();
    assert_eq!(result.len(), SCRATCH_BUF_SIZE);
}

#[test]
fn test_uninit_u8_array_all_elements_are_uninitialized() {
    let result = uninit_u8_array();
    for i in 0..SCRATCH_BUF_SIZE {
        assert!(result[i].as_ptr().is_null());
    }
}

#[test]
#[should_panic(expected = "uninitialized data")]
fn test_access_uninitialized_array_element_panics() {
    let result = uninit_u8_array();
    unsafe {
        let _val = result[0].assume_init(); // Attempting to assume_init on uninitialized data.
    }
}

