// Answer 0

#[derive(Debug)]
struct ClassUnicode {
    set: std::collections::HashSet<char>,
}

impl ClassUnicode {
    pub fn new(set: std::collections::HashSet<char>) -> Self {
        ClassUnicode { set }
    }
}

#[derive(Debug)]
struct CharacterClass {
    set: std::collections::HashSet<char>,
}

impl CharacterClass {
    pub fn new(set: std::collections::HashSet<char>) -> Self {
        CharacterClass { set }
    }

    pub fn difference(&mut self, other: &ClassUnicode) {
        self.set = self.set.difference(&other.set).cloned().collect();
    }
}

#[test]
fn test_difference_with_non_overlapping_classes() {
    let mut char_class = CharacterClass::new(std::collections::HashSet::from(['a', 'b', 'c']));
    let other_class = ClassUnicode::new(std::collections::HashSet::from(['d', 'e', 'f']));
    
    char_class.difference(&other_class);
    
    assert_eq!(char_class.set, std::collections::HashSet::from(['a', 'b', 'c']));
}

#[test]
fn test_difference_with_overlapping_classes() {
    let mut char_class = CharacterClass::new(std::collections::HashSet::from(['a', 'b', 'c']));
    let other_class = ClassUnicode::new(std::collections::HashSet::from(['b', 'c', 'd']));
    
    char_class.difference(&other_class);
    
    assert_eq!(char_class.set, std::collections::HashSet::from(['a']));
}

#[test]
fn test_difference_with_identical_classes() {
    let mut char_class = CharacterClass::new(std::collections::HashSet::from(['a', 'b', 'c']));
    let other_class = ClassUnicode::new(std::collections::HashSet::from(['a', 'b', 'c']));
    
    char_class.difference(&other_class);
    
    assert_eq!(char_class.set, std::collections::HashSet::new());
}

#[test]
fn test_difference_with_empty_other_class() {
    let mut char_class = CharacterClass::new(std::collections::HashSet::from(['a', 'b', 'c']));
    let other_class = ClassUnicode::new(std::collections::HashSet::new());
    
    char_class.difference(&other_class);
    
    assert_eq!(char_class.set, std::collections::HashSet::from(['a', 'b', 'c']));
}

