// Answer 0

#[derive(Debug)]
struct LiteralSet {
    literals: Vec<String>,
}

impl LiteralSet {
    fn new() -> Self {
        LiteralSet {
            literals: Vec::new(),
        }
    }
    
    fn _add_char_class(&mut self, cls: &hir::ClassUnicode, reverse: bool) -> bool {
        let class_length = cls.chars.len();
        if class_length > 255 {
            return false;
        }
        
        let mut new_literals = Vec::new();
        for literal in &self.literals {
            let mut extended_literal = literal.clone();
            if reverse {
                extended_literal.push_str(&cls.chars.chars().rev().collect::<String>());
            } else {
                extended_literal.push_str(&cls.chars);
            }
            new_literals.push(extended_literal);
        }
        self.literals = new_literals;
        true
    }
    
    fn add_char_class_reverse(&mut self, cls: &hir::ClassUnicode) -> bool {
        self._add_char_class(cls, true)
    }
}

mod hir {
    #[derive(Debug)]
    pub struct ClassUnicode {
        pub chars: String,
    }
}

#[test]
fn test_add_char_class_reverse() {
    let mut literal_set = LiteralSet::new();
    literal_set.literals.push("abc".to_string());
    
    let cls = hir::ClassUnicode {
        chars: "xyz".to_string(),
    };
    
    assert_eq!(literal_set.add_char_class_reverse(&cls), true);
    assert_eq!(literal_set.literals.len(), 1);
    assert_eq!(literal_set.literals[0], "abczyx");
}

#[test]
fn test_add_char_class_reverse_too_large() {
    let mut literal_set = LiteralSet::new();
    literal_set.literals.push("test".to_string());
    
    let cls = hir::ClassUnicode {
        chars: "abcdefghijklmnopqrstuvwxyza".repeat(10), // generates a string longer than 255 chars
    };
    
    assert_eq!(literal_set.add_char_class_reverse(&cls), false);
    assert_eq!(literal_set.literals.len(), 1);
    assert_eq!(literal_set.literals[0], "test");
}

