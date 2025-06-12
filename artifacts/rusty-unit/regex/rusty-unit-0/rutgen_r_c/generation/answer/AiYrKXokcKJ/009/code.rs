// Answer 0

#[test]
fn test_add_byte_class_with_multiple_ranges() {
    struct HirMock;
    impl Hir for HirMock {}

    let mut literals = Literals {
        lits: vec![Literal::new(vec![1, 2])],
        limit_size: 10,
        limit_class: 5,
    };

    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 1, end: 3 },
        ClassBytesRange { start: 5, end: 5 },
    ]);

    let result = literals.add_byte_class(&class_bytes);

    assert_eq!(result, true);
    assert_eq!(literals.lits.len(), 6); // 3 from first range + 1 from second range
}

#[test]
fn test_add_byte_class_with_single_range() {
    struct HirMock;
    impl Hir for HirMock {}

    let mut literals = Literals {
        lits: vec![Literal::new(vec![3])],
        limit_size: 10,
        limit_class: 5,
    };

    let class_bytes = ClassBytes::new(vec![
        ClassBytesRange { start: 2, end: 4 },
    ]);

    let result = literals.add_byte_class(&class_bytes);

    assert_eq!(result, true);
    assert_eq!(literals.lits.len(), 3); // 3 new literals created
}

#[test]
fn test_add_byte_class_with_no_ranges() {
    struct HirMock;
    impl Hir for HirMock {}

    let mut literals = Literals {
        lits: vec![Literal::new(vec![4])],
        limit_size: 10,
        limit_class: 5,
    };

    let class_bytes = ClassBytes::new(vec![]);

    let result = literals.add_byte_class(&class_bytes);

    assert_eq!(result, true);
    assert_eq!(literals.lits.len(), 1); // No new literals created
}

