// Answer 0

#[derive(Debug)]
struct ClassUnicode {
    set: UnicodeSet,
}

#[derive(Debug)]
struct UnicodeSet {
    characters: Vec<char>,
}

impl UnicodeSet {
    fn new(characters: Vec<char>) -> Self {
        UnicodeSet { characters }
    }

    fn intersect(&mut self, other: &UnicodeSet) {
        self.characters.retain(|c| other.characters.contains(c));
    }
}

impl ClassUnicode {
    fn new(characters: Vec<char>) -> Self {
        ClassUnicode {
            set: UnicodeSet::new(characters),
        }
    }

    pub fn intersect(&mut self, other: &ClassUnicode) {
        self.set.intersect(&other.set);
    }
}

#[test]
fn test_intersect_with_non_empty_classes() {
    let mut class1 = ClassUnicode::new(vec!['a', 'b', 'c', 'd']);
    let class2 = ClassUnicode::new(vec!['b', 'c', 'e', 'f']);
    
    class1.intersect(&class2);
    
    assert_eq!(class1.set.characters, vec!['b', 'c']);
}

#[test]
fn test_intersect_with_empty_class() {
    let mut class1 = ClassUnicode::new(vec!['a', 'b', 'c', 'd']);
    let class2 = ClassUnicode::new(vec![]);
    
    class1.intersect(&class2);
    
    assert_eq!(class1.set.characters, vec![]);
}

#[test]
fn test_intersect_with_identical_classes() {
    let mut class1 = ClassUnicode::new(vec!['x', 'y', 'z']);
    let class2 = ClassUnicode::new(vec!['x', 'y', 'z']);
    
    class1.intersect(&class2);
    
    assert_eq!(class1.set.characters, vec!['x', 'y', 'z']);
}

#[test]
fn test_intersect_with_no_common_characters() {
    let mut class1 = ClassUnicode::new(vec!['1', '2', '3']);
    let class2 = ClassUnicode::new(vec!['a', 'b', 'c']);
    
    class1.intersect(&class2);
    
    assert_eq!(class1.set.characters, vec![]);
}

