// Answer 0

#[test]
fn test_difference_non_empty_classes() {
    let mut class1 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
        ClassUnicodeRange { start: 'e', end: 'g' },
    ]);
    let class2 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'b', end: 'f' },
    ]);

    class1.difference(&class2);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'a' },
        ClassUnicodeRange { start: 'g', end: 'g' },
    ];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_difference_empty_class() {
    let mut class1 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class2 = ClassUnicode::empty();

    class1.difference(&class2);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ];
    
    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_difference_class_with_no_overlap() {
    let mut class1 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class2 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'd', end: 'f' },
    ]);

    class1.difference(&class2);
    
    let expected_ranges = vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ];

    assert_eq!(class1.ranges(), &expected_ranges);
}

#[test]
fn test_difference_identical_classes() {
    let mut class1 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);
    let class2 = ClassUnicode::new(vec![
        ClassUnicodeRange { start: 'a', end: 'c' },
    ]);

    class1.difference(&class2);
    
    let expected_ranges: Vec<ClassUnicodeRange> = vec![];

    assert_eq!(class1.ranges(), &expected_ranges);
}

