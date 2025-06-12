// Answer 0

#[test]
fn test_prefix_at_with_literal_prefix_found() {
    struct MockProg {
        prefixes: Vec<&'static str>,
    }

    impl MockProg {
        fn new(prefixes: Vec<&'static str>) -> Self {
            Self { prefixes }
        }

        fn find(&self, text: &[u8]) -> Option<(&'static str, usize)> {
            for &prefix in &self.prefixes {
                if text.starts_with(prefix.as_bytes()) {
                    return Some((prefix, prefix.len()));
                }
            }
            None
        }
    }

    struct DFA {
        prog: MockProg,
    }

    impl DFA {
        fn new(prog: MockProg) -> Self {
            Self { prog }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prog.find(&text[at..]).map(|(s, _)| at + s.len())
        }
    }

    let dfa = DFA::new(MockProg::new(vec!["abc", "def"]));
    assert_eq!(dfa.prefix_at(b"abcdef", 0), Some(3));
}

#[test]
fn test_prefix_at_with_no_literal_prefix_found() {
    struct MockProg {
        prefixes: Vec<&'static str>,
    }

    impl MockProg {
        fn new(prefixes: Vec<&'static str>) -> Self {
            Self { prefixes }
        }

        fn find(&self, text: &[u8]) -> Option<(&'static str, usize)> {
            for &prefix in &self.prefixes {
                if text.starts_with(prefix.as_bytes()) {
                    return Some((prefix, prefix.len()));
                }
            }
            None
        }
    }

    struct DFA {
        prog: MockProg,
    }

    impl DFA {
        fn new(prog: MockProg) -> Self {
            Self { prog }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prog.find(&text[at..]).map(|(s, _)| at + s.len())
        }
    }

    let dfa = DFA::new(MockProg::new(vec!["abc", "def"]));
    assert_eq!(dfa.prefix_at(b"ghijkl", 0), None);
}

#[test]
fn test_prefix_at_with_empty_text() {
    struct MockProg {
        prefixes: Vec<&'static str>,
    }

    impl MockProg {
        fn new(prefixes: Vec<&'static str>) -> Self {
            Self { prefixes }
        }

        fn find(&self, text: &[u8]) -> Option<(&'static str, usize)> {
            for &prefix in &self.prefixes {
                if text.starts_with(prefix.as_bytes()) {
                    return Some((prefix, prefix.len()));
                }
            }
            None
        }
    }

    struct DFA {
        prog: MockProg,
    }

    impl DFA {
        fn new(prog: MockProg) -> Self {
            Self { prog }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prog.find(&text[at..]).map(|(s, _)| at + s.len())
        }
    }

    let dfa = DFA::new(MockProg::new(vec!["abc", "def"]));
    assert_eq!(dfa.prefix_at(b"", 0), None);
}

#[test]
fn test_prefix_at_with_at_beyond_text_length() {
    struct MockProg {
        prefixes: Vec<&'static str>,
    }

    impl MockProg {
        fn new(prefixes: Vec<&'static str>) -> Self {
            Self { prefixes }
        }

        fn find(&self, text: &[u8]) -> Option<(&'static str, usize)> {
            for &prefix in &self.prefixes {
                if text.starts_with(prefix.as_bytes()) {
                    return Some((prefix, prefix.len()));
                }
            }
            None
        }
    }

    struct DFA {
        prog: MockProg,
    }

    impl DFA {
        fn new(prog: MockProg) -> Self {
            Self { prog }
        }

        fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
            self.prog.find(&text[at..]).map(|(s, _)| at + s.len())
        }
    }

    let dfa = DFA::new(MockProg::new(vec!["abc", "def"]));
    assert_eq!(dfa.prefix_at(b"abcdef", 10), None);
}

