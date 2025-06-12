// Answer 0

#[test]
fn test_union_non_empty_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    impl ClassUnicode {
        fn new(chars: Vec<char>) -> Self {
            Self {
                set: chars.into_iter().collect(),
            }
        }
    }

    let mut class1 = ClassUnicode::new(vec!['a', 'b', 'c']);
    let class2 = ClassUnicode::new(vec!['c', 'd', 'e']);
    
    class1.union(&class2);

    let expected: std::collections::HashSet<char> = ['a', 'b', 'c', 'd', 'e'].iter().cloned().collect();
    assert_eq!(class1.set, expected);
}

#[test]
fn test_union_empty_class() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    impl ClassUnicode {
        fn new(chars: Vec<char>) -> Self {
            Self {
                set: chars.into_iter().collect(),
            }
        }
    }

    let mut class1 = ClassUnicode::new(vec![]);
    let class2 = ClassUnicode::new(vec!['x', 'y', 'z']);
    
    class1.union(&class2);

    let expected: std::collections::HashSet<char> = ['x', 'y', 'z'].iter().cloned().collect();
    assert_eq!(class1.set, expected);
}

#[test]
fn test_union_identical_classes() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    impl ClassUnicode {
        fn new(chars: Vec<char>) -> Self {
            Self {
                set: chars.into_iter().collect(),
            }
        }
    }

    let mut class1 = ClassUnicode::new(vec!['1', '2', '3']);
    let class2 = ClassUnicode::new(vec!['1', '2', '3']);
    
    class1.union(&class2);

    let expected: std::collections::HashSet<char> = ['1', '2', '3'].iter().cloned().collect();
    assert_eq!(class1.set, expected);
}

#[test]
fn test_union_with_overlap() {
    struct ClassUnicode {
        set: std::collections::HashSet<char>,
    }

    impl ClassUnicode {
        fn new(chars: Vec<char>) -> Self {
            Self {
                set: chars.into_iter().collect(),
            }
        }
    }

    let mut class1 = ClassUnicode::new(vec!['A', 'B', 'C']);
    let class2 = ClassUnicode::new(vec!['B', 'C', 'D']);
    
    class1.union(&class2);

    let expected: std::collections::HashSet<char> = ['A', 'B', 'C', 'D'].iter().cloned().collect();
    assert_eq!(class1.set, expected);
}

