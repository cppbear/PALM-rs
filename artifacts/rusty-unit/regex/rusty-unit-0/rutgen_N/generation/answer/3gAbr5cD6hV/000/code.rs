// Answer 0

#[test]
fn test_new_class_unicode_empty() {
    struct ClassUnicodeRange;
    struct IntervalSet {
        ranges: Vec<ClassUnicodeRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self
        where
            I: IntoIterator<Item = ClassUnicodeRange>,
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    struct ClassUnicode {
        set: IntervalSet,
    }

    let ranges: Vec<ClassUnicodeRange> = vec![];
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(ranges),
    };

    assert_eq!(class_unicode.set.ranges.len(), 0);
}

#[test]
fn test_new_class_unicode_single_range() {
    struct ClassUnicodeRange;
    struct IntervalSet {
        ranges: Vec<ClassUnicodeRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self
        where
            I: IntoIterator<Item = ClassUnicodeRange>,
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    struct ClassUnicode {
        set: IntervalSet,
    }

    let ranges = vec![ClassUnicodeRange];
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(ranges),
    };

    assert_eq!(class_unicode.set.ranges.len(), 1);
}

#[test]
fn test_new_class_unicode_multiple_ranges() {
    struct ClassUnicodeRange;
    struct IntervalSet {
        ranges: Vec<ClassUnicodeRange>,
    }

    impl IntervalSet {
        fn new<I>(ranges: I) -> Self
        where
            I: IntoIterator<Item = ClassUnicodeRange>,
        {
            IntervalSet {
                ranges: ranges.into_iter().collect(),
            }
        }
    }

    struct ClassUnicode {
        set: IntervalSet,
    }

    let ranges = vec![ClassUnicodeRange, ClassUnicodeRange, ClassUnicodeRange];
    let class_unicode = ClassUnicode {
        set: IntervalSet::new(ranges),
    };

    assert_eq!(class_unicode.set.ranges.len(), 3);
}

