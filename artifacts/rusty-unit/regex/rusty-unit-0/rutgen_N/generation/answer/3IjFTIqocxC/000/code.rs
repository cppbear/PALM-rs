// Answer 0

#[derive(Debug)]
struct ClassBytes {
    set: std::collections::HashSet<u8>,
}

impl ClassBytes {
    pub fn new() -> Self {
        ClassBytes {
            set: std::collections::HashSet::new(),
        }
    }

    pub fn insert(&mut self, byte: u8) {
        self.set.insert(byte);
    }

    pub fn difference(&mut self, other: &ClassBytes) {
        self.set = self.set.difference(&other.set).cloned().collect();
    }
}

#[test]
fn test_difference_removes_common_elements() {
    let mut class_a = ClassBytes::new();
    class_a.insert(1);
    class_a.insert(2);
    class_a.insert(3);

    let class_b = ClassBytes::new();
    class_b.insert(2);
    class_b.insert(3);
    
    class_a.difference(&class_b);
    
    assert!(class_a.set.contains(&1));
    assert!(!class_a.set.contains(&2));
    assert!(!class_a.set.contains(&3));
}

#[test]
fn test_difference_removes_all_elements() {
    let mut class_a = ClassBytes::new();
    class_a.insert(1);
    class_a.insert(2);
    
    let class_b = ClassBytes::new();
    class_b.insert(1);
    class_b.insert(2);
    
    class_a.difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_difference_no_change_if_no_common_elements() {
    let mut class_a = ClassBytes::new();
    class_a.insert(1);
    class_a.insert(2);
    
    let class_b = ClassBytes::new();
    class_b.insert(3);
    class_b.insert(4);
    
    class_a.difference(&class_b);
    
    assert!(class_a.set.contains(&1));
    assert!(class_a.set.contains(&2));
}

#[test]
fn test_difference_empty_class_a() {
    let mut class_a = ClassBytes::new();
    
    let class_b = ClassBytes::new();
    class_b.insert(1);
    
    class_a.difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_difference_empty_class_b() {
    let mut class_a = ClassBytes::new();
    class_a.insert(1);
    
    let class_b = ClassBytes::new();
    
    class_a.difference(&class_b);
    
    assert!(class_a.set.contains(&1));
}

