// Answer 0

#[test]
fn test_difference_empty_classes() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class_a = ClassBytes { set: std::collections::HashSet::new() };
    let class_b = ClassBytes { set: std::collections::HashSet::new() };

    class_a.difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_difference_non_empty_classes() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class_a = ClassBytes { set: std::collections::HashSet::from([1, 2, 3, 4]) };
    let class_b = ClassBytes { set: std::collections::HashSet::from([2, 4]) };

    class_a.difference(&class_b);
    
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 3]));
}

#[test]
fn test_difference_all_elements_removed() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class_a = ClassBytes { set: std::collections::HashSet::from([1, 2, 3, 4]) };
    let class_b = ClassBytes { set: std::collections::HashSet::from([1, 2, 3, 4]) };

    class_a.difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_difference_no_elements_removed() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class_a = ClassBytes { set: std::collections::HashSet::from([1, 2, 3]) };
    let class_b = ClassBytes { set: std::collections::HashSet::from([4, 5]) };

    class_a.difference(&class_b);
    
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 2, 3]));
}

