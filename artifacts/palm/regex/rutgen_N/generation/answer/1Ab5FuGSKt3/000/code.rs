// Answer 0

#[test]
fn test_exec_anchored_start_true() {
    struct MockInputAt {
        position: usize,
        end: bool,
    }

    impl MockInputAt {
        fn new(position: usize, end: bool) -> Self {
            MockInputAt { position, end }
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.end
        }

        fn next_pos(&self) -> usize {
            self.position + 1
        }
    }

    struct MockProg {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct MockRegex {
        prog: MockProg,
    }

    impl MockRegex {
        fn clear(&mut self) {}
        
        fn backtrack(&self, _: MockInputAt) -> bool {
            true
        }
    }

    let mut regex = MockRegex {
        prog: MockProg {
            is_anchored_start: true,
            prefixes: vec![],
            matches: vec!["match".to_string()],
        },
    };

    let input_at = MockInputAt::new(0, false);
    assert!(regex.exec_(input_at));
}

#[test]
fn test_exec_anchored_start_false() {
    struct MockInputAt {
        position: usize,
        end: bool,
    }

    impl MockInputAt {
        fn new(position: usize, end: bool) -> Self {
            MockInputAt { position, end }
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.end
        }

        fn next_pos(&self) -> usize {
            self.position + 1
        }
    }

    struct MockProg {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct MockRegex {
        prog: MockProg,
    }

    impl MockRegex {
        fn clear(&mut self) {}
        
        fn backtrack(&self, _: MockInputAt) -> bool {
            false
        }
    }

    let mut regex = MockRegex {
        prog: MockProg {
            is_anchored_start: true,
            prefixes: vec![],
            matches: vec!["match".to_string()],
        },
    };

    let input_at = MockInputAt::new(1, false);
    assert!(!regex.exec_(input_at));
}

#[test]
fn test_exec_non_anchored_with_prefixes() {
    struct MockInputAt {
        position: usize,
        end: bool,
    }

    impl MockInputAt {
        fn new(position: usize, end: bool) -> Self {
            MockInputAt { position, end }
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.end
        }

        fn next_pos(&self) -> usize {
            self.position + 1
        }
    }

    struct MockProg {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct MockRegex {
        prog: MockProg,
    }

    impl MockRegex {
        fn clear(&mut self) {}
        
        fn backtrack(&self, at: MockInputAt) -> bool {
            at.position == 3
        }
    }

    let mut regex = MockRegex {
        prog: MockProg {
            is_anchored_start: false,
            prefixes: vec!["pre".to_string()],
            matches: vec!["match".to_string()],
        },
    };

    let input_at = MockInputAt::new(0, false);
    assert!(regex.exec_(input_at));
}

#[test]
fn test_exec_non_anchored_no_match() {
    struct MockInputAt {
        position: usize,
        end: bool,
    }

    impl MockInputAt {
        fn new(position: usize, end: bool) -> Self {
            MockInputAt { position, end }
        }

        fn is_start(&self) -> bool {
            self.position == 0
        }

        fn is_end(&self) -> bool {
            self.end
        }

        fn next_pos(&self) -> usize {
            self.position + 1
        }
    }

    struct MockProg {
        is_anchored_start: bool,
        prefixes: Vec<String>,
        matches: Vec<String>,
    }

    struct MockRegex {
        prog: MockProg,
    }

    impl MockRegex {
        fn clear(&mut self) {}
        
        fn backtrack(&self, _: MockInputAt) -> bool {
            false
        }
    }

    let mut regex = MockRegex {
        prog: MockProg {
            is_anchored_start: false,
            prefixes: vec!["pre".to_string()],
            matches: vec!["nosuccess".to_string()],
        },
    };

    let input_at = MockInputAt::new(0, false);
    assert!(!regex.exec_(input_at));
}

