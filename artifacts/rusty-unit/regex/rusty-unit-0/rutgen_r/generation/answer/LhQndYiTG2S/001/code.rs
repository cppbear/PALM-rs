// Answer 0

#[test]
fn test_find_literals_unanchored() {
    struct DummyNFA {
        prefixes: Vec<(usize, usize)>,
        suffixes: Vec<(usize, usize)>,
        is_anchored_start: bool,
    }

    struct DummyRegex {
        ro: DummyNFA,
    }

    impl DummyNFA {
        fn find(&self, text: &[u8]) -> Option<(usize, usize)> {
            for (s, e) in &self.prefixes {
                let pattern = &text[*s..*e];
                if text.contains(&pattern) {
                    return Some((*s, *e));
                }
            }
            None
        }

        fn find_start(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }

        fn find_end(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }
    }

    let nfa = DummyNFA {
        prefixes: vec![(0, 3), (4, 7)],
        suffixes: vec![],
        is_anchored_start: false,
    };
    
    let regex = DummyRegex { ro: nfa };
    
    let result = regex.find_literals(MatchLiteralType::Unanchored, b"hello world", 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_find_literals_anchored_end() {
    struct DummyNFA {
        prefixes: Vec<(usize, usize)>,
        suffixes: Vec<(usize, usize)>,
        is_anchored_start: bool,
    }

    struct DummyRegex {
        ro: DummyNFA,
    }

    impl DummyNFA {
        fn find(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }

        fn find_start(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }

        fn find_end(&self, text: &[u8]) -> Option<(usize, usize)> {
            // Assume we have a literal "world" at the end of our string
            if text.ends_with(b"world") {
                return Some((text.len() - 5, text.len()));
            }
            None
        }
    }

    let nfa = DummyNFA {
        prefixes: vec![],
        suffixes: vec![(0, 5)],
        is_anchored_start: false,
    };
    
    let regex = DummyRegex { ro: nfa };
    
    let result = regex.find_literals(MatchLiteralType::AnchoredEnd, b"hello world", 0);
    assert_eq!(result, Some((6, 11)));
}

#[test]
#[should_panic]
fn test_find_literals_anchored_end_panic() {
    struct DummyNFA {
        prefixes: Vec<(usize, usize)>,
        suffixes: Vec<(usize, usize)>,
        is_anchored_start: bool,
    }

    struct DummyRegex {
        ro: DummyNFA,
    }

    impl DummyNFA {
        fn find(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }

        fn find_start(&self, _text: &[u8]) -> Option<(usize, usize)> {
            None
        }

        fn find_end(&self, text: &[u8]) -> Option<(usize, usize)> {
            if text.len() < 5 {
                panic!("Slice out of bounds");
            }
            None
        }
    }

    let nfa = DummyNFA {
        prefixes: vec![],
        suffixes: vec![(0, 5)],
        is_anchored_start: false,
    };
    
    let regex = DummyRegex { ro: nfa };
    
    // This will trigger the panic in find_end
    let _result = regex.find_literals(MatchLiteralType::AnchoredEnd, b"hi", 0);
}

