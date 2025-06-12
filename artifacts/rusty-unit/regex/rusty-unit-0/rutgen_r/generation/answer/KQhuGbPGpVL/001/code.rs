// Answer 0

#[derive(Debug)]
struct ClassBytes {
    set: std::collections::HashSet<u8>,
}

impl ClassBytes {
    fn new(elements: Vec<u8>) -> Self {
        let set: std::collections::HashSet<u8> = elements.into_iter().collect();
        ClassBytes { set }
    }
}

impl ClassBytes {
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {
        self.set.symmetric_difference(&other.set);
    }
}

#[test]
fn test_symmetric_difference_no_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![4, 5, 6]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 2, 3, 4, 5, 6]));
}

#[test]
fn test_symmetric_difference_all_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![1, 2, 3]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 0);
}

#[test]
fn test_symmetric_difference_partial_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![2, 3, 4, 5]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 4, 5]));
}

#[test]
fn test_symmetric_difference_empty_a() {
    let mut class_a = ClassBytes::new(vec![]);
    let class_b = ClassBytes::new(vec![1, 2, 3]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 2, 3]));
}

#[test]
fn test_symmetric_difference_empty_b() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set, std::collections::HashSet::from([1, 2, 3]));
}

