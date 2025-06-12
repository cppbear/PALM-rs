// Answer 0

#[derive(Debug)]
struct ClassUnicode {
    set: std::collections::HashSet<char>,
}

impl ClassUnicode {
    pub fn new(chars: &[char]) -> Self {
        let set: std::collections::HashSet<char> = chars.iter().copied().collect();
        ClassUnicode { set }
    }
    
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {
        let sym_diff: std::collections::HashSet<char> = 
            self.set.symmetric_difference(&other.set).cloned().collect();
        self.set = sym_diff;
    }
}

#[test]
fn test_symmetric_difference_no_common_elements() {
    let mut class_a = ClassUnicode::new(&['a', 'b', 'c']);
    let class_b = ClassUnicode::new(&['d', 'e', 'f']);
    
    class_a.symmetric_difference(&class_b);
    
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'b', 'c', 'd', 'e', 'f']));
}

#[test]
fn test_symmetric_difference_some_common_elements() {
    let mut class_a = ClassUnicode::new(&['a', 'b', 'c']);
    let class_b = ClassUnicode::new(&['b', 'c', 'd']);
    
    class_a.symmetric_difference(&class_b);
    
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'd']));
}

#[test]
fn test_symmetric_difference_all_common_elements() {
    let mut class_a = ClassUnicode::new(&['a', 'b', 'c']);
    let class_b = ClassUnicode::new(&['a', 'b', 'c']);
    
    class_a.symmetric_difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_symmetric_difference_empty_classes() {
    let mut class_a = ClassUnicode::new(&[]);
    let class_b = ClassUnicode::new(&[]);
    
    class_a.symmetric_difference(&class_b);
    
    assert!(class_a.set.is_empty());
}

#[test]
fn test_symmetric_difference_one_empty_class() {
    let mut class_a = ClassUnicode::new(&['a', 'b', 'c']);
    let class_b = ClassUnicode::new(&[]);
    
    class_a.symmetric_difference(&class_b);
    
    assert_eq!(class_a.set, std::collections::HashSet::from(['a', 'b', 'c']));
}

