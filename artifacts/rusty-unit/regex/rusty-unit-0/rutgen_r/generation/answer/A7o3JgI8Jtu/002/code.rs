// Answer 0

fn test_case_fold_simple_unicode() {
    struct UnicodeClass {
        characters: Vec<char>,
    }

    impl UnicodeClass {
        fn case_fold_simple(&mut self) {
            // Simulating case folding operation for testing purpose
            for c in &mut self.characters {
                if c.is_ascii_uppercase() {
                    *c = c.to_ascii_lowercase();
                }
            }
        }
    }

    enum Class {
        Unicode(UnicodeClass),
        Bytes,
    }

    let mut unicode_class_instance = UnicodeClass {
        characters: vec!['A', 'B', 'C', 'a', 'b', 'c'],
    };
    
    let mut class_instance = Class::Unicode(unicode_class_instance);

    if let Class::Unicode(ref mut x) = class_instance {
        x.case_fold_simple();
    }

    if let Class::Unicode(x) = class_instance {
        assert_eq!(x.characters, vec!['a', 'b', 'c', 'a', 'b', 'c']);
    }
}

fn test_case_fold_simple_unicode_boundary() {
    struct UnicodeClass {
        characters: Vec<char>,
    }

    impl UnicodeClass {
        fn case_fold_simple(&mut self) {
            // Simulating case folding operation for testing purpose
            for c in &mut self.characters {
                if c.is_ascii_uppercase() {
                    *c = c.to_ascii_lowercase();
                }
            }
        }
    }

    enum Class {
        Unicode(UnicodeClass),
        Bytes,
    }

    let mut unicode_boundary_class_instance = UnicodeClass {
        characters: vec!['Z', 'Y', 'X', 'a', 'b', 'c'],
    };
    
    let mut class_instance = Class::Unicode(unicode_boundary_class_instance);

    if let Class::Unicode(ref mut x) = class_instance {
        x.case_fold_simple();
    }

    if let Class::Unicode(x) = class_instance {
        assert_eq!(x.characters, vec!['z', 'y', 'x', 'a', 'b', 'c']);
    }
}

