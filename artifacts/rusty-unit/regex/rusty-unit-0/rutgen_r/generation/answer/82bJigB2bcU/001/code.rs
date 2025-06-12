// Answer 0

#[test]
fn test_add_char_class_success() {
    struct TestSet {
        literals: Vec<char>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                literals: Vec::new(),
            }
        }

        fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {
            // Simulating the function from the original context
            // for this test function. Replace with actual logic if available.
            if cls.is_large() {
                return false; // Simulate the condition where class is too big.
            }
            self.literals.extend(cls.characters());
            true
        }
    }

    struct ClassUnicode {
        characters: Vec<char>,
        large: bool,
    }

    impl ClassUnicode {
        fn new(characters: Vec<char>, large: bool) -> Self {
            ClassUnicode { characters, large }
        }

        fn characters(&self) -> &[char] {
            &self.characters
        }

        fn is_large(&self) -> bool {
            self.large
        }
    }

    let mut set = TestSet::new();
    let char_class = ClassUnicode::new(vec!['a', 'b', 'c'], false);
    assert!(set.add_char_class(&char_class));
    assert_eq!(set.literals, vec!['a', 'b', 'c']);
}

#[test]
fn test_add_char_class_failure() {
    struct TestSet {
        literals: Vec<char>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet {
                literals: Vec::new(),
            }
        }

        fn add_char_class(&mut self, cls: &hir::ClassUnicode) -> bool {
            // Simulating the function from the original context
            // for this test function. Replace with actual logic if available.
            if cls.is_large() {
                return false; // Simulate the condition where class is too big.
            }
            self.literals.extend(cls.characters());
            true
        }
    }

    struct ClassUnicode {
        characters: Vec<char>,
        large: bool,
    }

    impl ClassUnicode {
        fn new(characters: Vec<char>, large: bool) -> Self {
            ClassUnicode { characters, large }
        }

        fn characters(&self) -> &[char] {
            &self.characters
        }

        fn is_large(&self) -> bool {
            self.large
        }
    }

    let mut set = TestSet::new();
    let char_class = ClassUnicode::new(vec!['a', 'b', 'c'], true);
    assert!(!set.add_char_class(&char_class));
    assert!(set.literals.is_empty());
}

