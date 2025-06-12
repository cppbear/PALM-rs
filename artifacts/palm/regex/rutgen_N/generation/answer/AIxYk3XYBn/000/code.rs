// Answer 0

#[derive(Debug)]
struct ClassUnicode {
    set: std::collections::HashSet<char>,
}

impl ClassUnicode {
    pub fn new(chars: Vec<char>) -> Self {
        ClassUnicode {
            set: chars.into_iter().collect(),
        }
    }
}

impl ClassUnicode {
    pub fn union(&mut self, other: &ClassUnicode) {
        self.set.extend(&other.set);
    }
}

#[test]
fn test_union_with_empty_set() {
    let mut class1 = ClassUnicode::new(vec![]);
    let class2 = ClassUnicode::new(vec!['a', 'b', 'c']);
    
    class1.union(&class2);
    
    assert_eq!(class1.set.len(), 3);
    assert!(class1.set.contains(&'a'));
    assert!(class1.set.contains(&'b'));
    assert!(class1.set.contains(&'c'));
}

#[test]
fn test_union_with_non_empty_set() {
    let mut class1 = ClassUnicode::new(vec!['a', 'b']);
    let class2 = ClassUnicode::new(vec!['b', 'c', 'd']);
    
    class1.union(&class2);
    
    assert_eq!(class1.set.len(), 4);
    assert!(class1.set.contains(&'a'));
    assert!(class1.set.contains(&'b'));
    assert!(class1.set.contains(&'c'));
    assert!(class1.set.contains(&'d'));
}

#[test]
fn test_union_with_identical_set() {
    let mut class1 = ClassUnicode::new(vec!['a', 'b', 'c']);
    let class2 = ClassUnicode::new(vec!['a', 'b', 'c']);
    
    class1.union(&class2);
    
    assert_eq!(class1.set.len(), 3);
    assert!(class1.set.contains(&'a'));
    assert!(class1.set.contains(&'b'));
    assert!(class1.set.contains(&'c'));
}

#[test]
fn test_union_with_no_common_elements() {
    let mut class1 = ClassUnicode::new(vec!['x', 'y']);
    let class2 = ClassUnicode::new(vec!['a', 'b']);
    
    class1.union(&class2);
    
    assert_eq!(class1.set.len(), 4);
    assert!(class1.set.contains(&'x'));
    assert!(class1.set.contains(&'y'));
    assert!(class1.set.contains(&'a'));
    assert!(class1.set.contains(&'b'));
}

