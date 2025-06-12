// Answer 0

#[derive(Debug, Default)]
struct ClassBytes {
    set: std::collections::HashSet<u8>,
}

impl ClassBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        let set: std::collections::HashSet<u8> = bytes.into_iter().collect();
        ClassBytes { set }
    }

    pub fn union(&mut self, other: &ClassBytes) {
        self.set.extend(other.set.iter());
    }
}

#[test]
fn test_union_empty_classes() {
    let mut class_a = ClassBytes::new(vec![]);
    let class_b = ClassBytes::new(vec![]);
    class_a.union(&class_b);
    assert!(class_a.set.is_empty());
}

#[test]
fn test_union_with_empty_class() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![]);
    class_a.union(&class_b);
    assert_eq!(class_a.set.len(), 3);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
    assert!(class_a.set.contains(&3));
}

#[test]
fn test_union_with_non_empty_classes() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![3, 4, 5]);
    class_a.union(&class_b);
    assert_eq!(class_a.set.len(), 5);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
    assert!(class_a.set.contains(&3));
    assert!(class_a.set.contains(&4));
    assert!(class_a.set.contains(&5));
}

#[test]
fn test_union_identical_classes() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![1, 2, 3]);
    class_a.union(&class_b);
    assert_eq!(class_a.set.len(), 3);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
    assert!(class_a.set.contains(&3));
}

