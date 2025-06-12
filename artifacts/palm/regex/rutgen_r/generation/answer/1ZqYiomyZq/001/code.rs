// Answer 0

#[test]
fn test_difference_with_empty_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class1 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    let class2 = ClassUnicode {
        set: std::collections::HashSet::new(),
    };
    
    class1.difference(&class2);
    assert_eq!(class1.set, std::collections::HashSet::from(['a', 'b', 'c']));
}

#[test]
fn test_difference_with_identical_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class1 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    let class2 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    
    class1.difference(&class2);
    assert_eq!(class1.set, std::collections::HashSet::new());
}

#[test]
fn test_difference_with_subset_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class1 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    let class2 = ClassUnicode {
        set: std::collections::HashSet::from(['b']),
    };
    
    class1.difference(&class2);
    assert_eq!(class1.set, std::collections::HashSet::from(['a', 'c']));
}

#[test]
fn test_difference_with_non_overlapping_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class1 = ClassUnicode {
        set: std::collections::HashSet::from(['x', 'y', 'z']),
    };
    let class2 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    
    class1.difference(&class2);
    assert_eq!(class1.set, std::collections::HashSet::from(['x', 'y', 'z']));
}

#[test]
fn test_difference_with_all_characters() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    let mut class1 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c']),
    };
    let class2 = ClassUnicode {
        set: std::collections::HashSet::from(['a', 'b', 'c', 'd', 'e']),
    };
    
    class1.difference(&class2);
    assert_eq!(class1.set, std::collections::HashSet::new());
}

