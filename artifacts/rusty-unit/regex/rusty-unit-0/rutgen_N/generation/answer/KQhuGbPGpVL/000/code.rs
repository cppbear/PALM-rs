// Answer 0

#[derive(Debug)]
struct ClassBytes {
    set: std::collections::HashSet<u8>,
}

impl ClassBytes {
    pub fn new(elements: Vec<u8>) -> Self {
        ClassBytes {
            set: elements.into_iter().collect(),
        }
    }
    
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {
        let diff: std::collections::HashSet<u8> = self.set.symmetric_difference(&other.set).cloned().collect();
        self.set = diff; // Update self.set to contain the symmetric difference
    }
}

#[test]
fn test_symmetric_difference_no_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![4, 5, 6]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 6);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
    assert!(class_a.set.contains(&3));
    assert!(class_a.set.contains(&4));
    assert!(class_a.set.contains(&5));
    assert!(class_a.set.contains(&6));
}

#[test]
fn test_symmetric_difference_full_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![1, 2, 3]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 0);
}

#[test]
fn test_symmetric_difference_partial_overlap() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![2, 3, 4]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 4);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&4));
    assert!(!class_a.set.contains(&2));
    assert!(!class_a.set.contains(&3));
}

#[test]
fn test_symmetric_difference_empty_class_b() {
    let mut class_a = ClassBytes::new(vec![1, 2, 3]);
    let class_b = ClassBytes::new(vec![]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 3);
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
    assert!(class_a.set.contains(&3));
}

#[test]
fn test_symmetric_difference_empty_class_a() {
    let mut class_a = ClassBytes::new(vec![]);
    let class_b = ClassBytes::new(vec![4, 5, 6]);
    class_a.symmetric_difference(&class_b);
    assert_eq!(class_a.set.len(), 3);
    assert!(class_a.set.contains(&4));
    assert!(class_a.set.contains(&5));
    assert!(class_a.set.contains(&6));
}

