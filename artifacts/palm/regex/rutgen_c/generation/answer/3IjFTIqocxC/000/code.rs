// Answer 0

#[test]
fn test_difference_with_non_empty_sets() {
    struct DummyInterval;

    impl DummyInterval {
        fn lower(&self) -> u8 { 0 }
        fn upper(&self) -> u8 { 255 }
        fn is_intersection_empty(&self, _: &Self) -> bool { false }
        fn difference(&self, _: &Self) -> (Option<Self>, Option<Self>) {
            (Some(*self), None)
        }
    }

    let class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
        ClassBytesRange { start: 20, end: 30 },
    ]);

    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 5, end: 15 },
    ]);

    let mut class_bytes_a = class_a.clone();
    class_bytes_a.difference(&class_b);
    
    assert_eq!(class_bytes_a.ranges(), &[ClassBytesRange { start: 0, end: 4 }, ClassBytesRange { start: 20, end: 30 }]);
}

#[test]
fn test_difference_with_empty_set() {
    let class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);

    let class_b = ClassBytes::empty();

    let mut class_bytes_a = class_a.clone();
    class_bytes_a.difference(&class_b);

    assert_eq!(class_bytes_a.ranges(), &[ClassBytesRange { start: 0, end: 10 }]);
}

#[test]
fn test_difference_with_identical_sets() {
    let class_a = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);

    let class_b = ClassBytes::new(vec![
        ClassBytesRange { start: 0, end: 10 },
    ]);

    let mut class_bytes_a = class_a.clone();
    class_bytes_a.difference(&class_b);

    assert_eq!(class_bytes_a.ranges(), &[]);
}

