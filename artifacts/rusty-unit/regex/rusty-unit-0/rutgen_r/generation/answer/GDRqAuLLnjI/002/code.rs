// Answer 0

fn test_negate_unicode_class() {
    struct MockUnicodeClass {
        characters: Vec<char>,
    }

    impl MockUnicodeClass {
        fn new(chars: Vec<char>) -> Self {
            Self { characters: chars }
        }

        fn negate(&mut self) {
            // Logic to negate the character set, just for testing purposes
            let all_chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
            self.characters = all_chars
                .into_iter()
                .filter(|c| !self.characters.contains(c))
                .collect();
        }
    }

    enum Class {
        Unicode(MockUnicodeClass),
        Bytes,
    }

    impl Class {
        pub fn negate(&mut self) {
            match *self {
                Class::Unicode(ref mut x) => x.negate(),
                Class::Bytes => panic!("Bytes class cannot be negated"),
            }
        }
    }

    let mut unicode_class = Class::Unicode(MockUnicodeClass::new(vec!['a', 'b', 'c']));
    unicode_class.negate();

    if let Class::Unicode(mock_unicode) = unicode_class {
        assert!(mock_unicode.characters.contains(&'d')); // checking that 'd' is now included
        assert!(!mock_unicode.characters.contains(&'a')); // checking 'a' is excluded
        assert!(!mock_unicode.characters.contains(&'b')); // checking 'b' is excluded
        assert!(!mock_unicode.characters.contains(&'c')); // checking 'c' is excluded
    } else {
        panic!("The class should still be Unicode.");
    }
}

