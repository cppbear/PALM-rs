// Answer 0

#[test]
fn test_intersect_empty_classes() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class1 = ClassBytes { set: std::collections::HashSet::new() };
    let class2 = ClassBytes { set: std::collections::HashSet::new() };

    class1.intersect(&class2);
    assert!(class1.set.is_empty());
}

#[test]
fn test_intersect_disjoint_classes() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class1 = ClassBytes { set: {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    }};
    let class2 = ClassBytes { set: {
        let mut set = std::collections::HashSet::new();
        set.insert(4);
        set.insert(5);
        set.set
    }};

    class1.intersect(&class2);
    assert!(class1.set.is_empty());
}

#[test]
fn test_intersect_same_classes() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class1 = ClassBytes { set: {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    }};
    let class2 = ClassBytes { set: class1.set.clone() };

    class1.intersect(&class2);
    assert_eq!(class1.set.len(), 3);
}

#[test]
fn test_intersect_partial_overlap() {
    struct ClassBytes {
        set: std::collections::HashSet<u8>,
    }

    let mut class1 = ClassBytes { set: {
        let mut set = std::collections::HashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    }};
    let class2 = ClassBytes { set: {
        let mut set = std::collections::HashSet::new();
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set
    }};

    class1.intersect(&class2);
    assert_eq!(class1.set.len(), 2);
    assert!(class1.set.contains(&2));
    assert!(class1.set.contains(&3));
}

