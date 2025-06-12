// Answer 0

#[test]
fn test_push_valid_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 10, end: 20 };
    class_bytes.push(range);
}

#[test]
fn test_push_single_point_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 100, end: 100 };
    class_bytes.push(range);
}

#[test]
fn test_push_full_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 0, end: 255 };
    class_bytes.push(range);
}

#[test]
fn test_push_multiple_ranges() {
    let mut class_bytes = ClassBytes::new(vec![]);
    class_bytes.push(ClassBytesRange { start: 5, end: 15 });
    class_bytes.push(ClassBytesRange { start: 30, end: 60 });
    class_bytes.push(ClassBytesRange { start: 200, end: 255 });
}

#[test]
#[should_panic]
fn test_push_reverse_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 50, end: 30 }; // start > end
    class_bytes.push(range);
}

#[test]
fn test_push_zero_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 0, end: 0 };
    class_bytes.push(range);
}

#[test]
fn test_push_large_range() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 250, end: 255 };
    class_bytes.push(range);
}

#[test]
fn test_push_edge_case() {
    let mut class_bytes = ClassBytes::new(vec![]);
    let range = ClassBytesRange { start: 255, end: 255 }; // maximum start and end
    class_bytes.push(range);
}

