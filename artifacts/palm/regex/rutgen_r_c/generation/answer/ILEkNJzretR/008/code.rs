// Answer 0

#[test]
fn test_c_bytes_non_empty() {
    struct TestStruct {
        compiled: Program,
    }

    impl TestStruct {
        fn c_bytes(&mut self, bytes: &[u8]) -> Result {
            debug_assert!(!bytes.is_empty());
            let mut bytes: Box<Iterator<Item=&u8>> =
                if self.compiled.is_reverse {
                    Box::new(bytes.iter().rev())
                } else {
                    Box::new(bytes.iter())
                };
            let first = *bytes.next().expect("non-empty literal");
            let Patch { mut hole, entry } = self.c_byte(first)?;
            for &b in bytes {
                let p = self.c_byte(b)?;
                self.fill(hole, p.entry);
                hole = p.hole;
            }
            Ok(Patch { hole: hole, entry: entry })
        }

        fn c_byte(&mut self, _b: u8) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr::default(),
            })
        }

        fn fill(&mut self, _hole: Hole, _goto: InstPtr) {
        }
    }

    let mut test_struct = TestStruct {
        compiled: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: InstPtr::default(),
            byte_classes: vec![],
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
    };

    let input_bytes: &[u8] = &[1, 2, 3, 4, 5];
    let result = test_struct.c_bytes(input_bytes);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "non-empty literal")]
fn test_c_bytes_empty() {
    struct TestStruct {
        compiled: Program,
    }

    impl TestStruct {
        fn c_bytes(&mut self, bytes: &[u8]) -> Result {
            debug_assert!(!bytes.is_empty());
            let mut bytes: Box<Iterator<Item=&u8>> =
                if self.compiled.is_reverse {
                    Box::new(bytes.iter().rev())
                } else {
                    Box::new(bytes.iter())
                };
            let first = *bytes.next().expect("non-empty literal");
            let Patch { mut hole, entry } = self.c_byte(first)?;
            for &b in bytes {
                let p = self.c_byte(b)?;
                self.fill(hole, p.entry);
                hole = p.hole;
            }
            Ok(Patch { hole: hole, entry: entry })
        }

        fn c_byte(&mut self, _b: u8) -> Result {
            Ok(Patch {
                hole: Hole::None,
                entry: InstPtr::default(),
            })
        }

        fn fill(&mut self, _hole: Hole, _goto: InstPtr) {
        }
    }

    let mut test_struct = TestStruct {
        compiled: Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: InstPtr::default(),
            byte_classes: vec![],
            only_utf8: false,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::default(),
            dfa_size_limit: 0,
        },
    };

    let input_bytes: &[u8] = &[];
    test_struct.c_bytes(input_bytes);
}


