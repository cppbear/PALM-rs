// Answer 0

#[test]
fn test_caps_valid_index() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Set slot size for tests
    threads.caps = vec![Some(1), Some(2), Some(3), Some(4)]; // Initialize captures

    let result = threads.caps(1); // Should not panic, valid index (1 * 2 = 2)
    assert_eq!(result, &mut [Some(3), Some(4)]); // Check returned slice
}

#[test]
fn test_caps_zero_index() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Set slot size for tests
    threads.caps = vec![Some(1), Some(2)]; // Initialize captures

    let result = threads.caps(0); // Should not panic, valid index (0 * 2 = 0)
    assert_eq!(result, &mut [Some(1), Some(2)]); // Check returned slice
}

#[should_panic]
fn test_caps_out_of_bounds() {
    let mut threads = Threads::new();
    threads.slots_per_thread = 2; // Set slot size for tests
    threads.caps = vec![Some(1), Some(2)]; // Initialize captures

    let _ = threads.caps(2); // Should panic, invalid index (2 * 2 = 4, out of bounds)
}

