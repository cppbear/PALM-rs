// Answer 0

#[test]
fn test_c_literal_reversed_with_valid_characters() {
    struct DummyCompiler {
        compiled: Program,
    }

    impl DummyCompiler {
        fn new() -> Self {
            Self {
                compiled: Program {
                    insts: vec![],
                    matches: vec![],
                    captures: vec![],
                    capture_name_idx: Arc::new(HashMap::new()),
                    start: InstPtr(0),
                    byte_classes: vec![],
                    only_utf8: true,
                    is_bytes: false,
                    is_dfa: false,
                    is_reverse: true,
                    is_anchored_start: false,
                    is_anchored_end: false,
                    has_unicode_word_boundary: false,
                    prefixes: LiteralSearcher::default(),
                    dfa_size_limit: 0,
                },
            }
        }

        fn c_char(&mut self, _c: char) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr(0),
            })
        }

        fn fill(&mut self, _hole: Hole, _goto: InstPtr) {}
    }

    let mut compiler = DummyCompiler::new();
    let chars = vec!['a', 'b', 'c'];

    let result = compiler.c_literal(&chars);

    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::None));
}

#[test]
#[should_panic(expected = "non-empty literal")]
fn test_c_literal_panics_on_empty_chars() {
    struct DummyCompiler {
        compiled: Program,
    }
    
    impl DummyCompiler {
        fn new() -> Self {
            Self {
                compiled: Program {
                    insts: vec![],
                    matches: vec![],
                    captures: vec![],
                    capture_name_idx: Arc::new(HashMap::new()),
                    start: InstPtr(0),
                    byte_classes: vec![],
                    only_utf8: true,
                    is_bytes: false,
                    is_dfa: false,
                    is_reverse: true,
                    is_anchored_start: false,
                    is_anchored_end: false,
                    has_unicode_word_boundary: false,
                    prefixes: LiteralSearcher::default(),
                    dfa_size_limit: 0,
                },
            }
        }

        fn c_char(&mut self, _c: char) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr(0),
            })
        }

        fn fill(&mut self, _hole: Hole, _goto: InstPtr) {}
    }

    let mut compiler = DummyCompiler::new();
    let chars: Vec<char> = vec![];

    compiler.c_literal(&chars);
}

