// Answer 0

#[test]
fn test_symmetric_difference_empty_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class_a = ClassUnicode {
        set: std::collections::HashSet::new(),
    };
    let class_b = ClassUnicode {
        set: std::collections::HashSet::new(),
    };

    class_a.symmetric_difference(&class_b);
    assert!(class_a.set.is_empty());
}

#[test]
fn test_symmetric_difference_disjoint_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class_a = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b']),
    };
    let class_b = ClassUnicode {
        set: std::collections::HashSet::from(['c', 'd']),
    };

    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'b', 'c', 'd']));
}

#[test]
fn test_symmetric_difference_overlapping_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class_a = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    let class_b = ClassUnicode {
        set: std::collections::HashSet::from(['b', 'c', 'd']),
    };

    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'd']));
}

#[test]
fn test_symmetric_difference_identical_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class_a = ClassUnicode {
        set: std::collections::HashSet::from(['x', 'y', 'z']),
    };
    let class_b = ClassUnicode {
        set: std::collections::HashSet::from(['x', 'y', 'z']),
    };

    class_a.symmetric_difference(&class_b);
    assert!(class_a.set.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class_a = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'e', 'i']),
    };
    let class_b = ClassUnicode {
        set: std::collections::HashSet::new(),
    };

    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'e', 'i']));
}

