// Answer 0

#[test]
fn test_unwrap_class_bytes_valid() {
    let range = ClassBytesRange { start: 0, end: 255 }; // Valid byte range
    let set = IntervalSet::new(vec![range].into_iter().collect()); // Initialize set
    let cls = ClassBytes { set }; // Create ClassBytes instance
    let frame = HirFrame::ClassBytes(cls); // Create HirFrame instance containing ClassBytes

    let _ = frame.unwrap_class_bytes(); // Call the function under test
}

#[test]
fn test_unwrap_class_bytes_empty_set() {
    let set = IntervalSet::new(vec![].into_iter().collect()); // Empty set
    let cls = ClassBytes { set }; // Create ClassBytes instance
    let frame = HirFrame::ClassBytes(cls); // Create HirFrame instance containing ClassBytes

    let _ = frame.unwrap_class_bytes(); // Call the function under test
}

#[test]
fn test_unwrap_class_bytes_single_range() {
    let range = ClassBytesRange { start: 1, end: 1 }; // Single byte range
    let set = IntervalSet::new(vec![range].into_iter().collect()); // Initialize set
    let cls = ClassBytes { set }; // Create ClassBytes instance
    let frame = HirFrame::ClassBytes(cls); // Create HirFrame instance containing ClassBytes

    let _ = frame.unwrap_class_bytes(); // Call the function under test
}

