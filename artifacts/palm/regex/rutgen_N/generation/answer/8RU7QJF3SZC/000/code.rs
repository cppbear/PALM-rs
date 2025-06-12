// Answer 0

#[test]
fn test_fmt_with_empty_table() {
    struct DummyDFA {
        num_byte_classes: usize,
        table: Vec<u8>,
    }

    impl DummyDFA {
        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    let dfa = DummyDFA {
        num_byte_classes: 1,
        table: vec![],
    };
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_with_single_state() {
    struct DummyDFA {
        num_byte_classes: usize,
        table: Vec<u8>,
    }

    impl DummyDFA {
        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    struct TransitionsRow<'a>(&'a [u8]);
    
    impl std::fmt::Debug for DummyDFA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut fmtd = f.debug_map();
            for si in 0..self.num_states() {
                let s = si * self.num_byte_classes;
                let e = s + self.num_byte_classes;
                fmtd.entry(&si.to_string(), &TransitionsRow(&self.table[s..e]));
            }
            fmtd.finish()
        }
    }

    let dfa = DummyDFA {
        num_byte_classes: 1,
        table: vec![1],
    };
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{ \"0\": [1] }");
}

#[test]
fn test_fmt_with_multiple_states() {
    struct DummyDFA {
        num_byte_classes: usize,
        table: Vec<u8>,
    }

    impl DummyDFA {
        fn num_states(&self) -> usize {
            self.table.len() / self.num_byte_classes
        }
    }

    struct TransitionsRow<'a>(&'a [u8]);
    
    impl std::fmt::Debug for DummyDFA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut fmtd = f.debug_map();
            for si in 0..self.num_states() {
                let s = si * self.num_byte_classes;
                let e = s + self.num_byte_classes;
                fmtd.entry(&si.to_string(), &TransitionsRow(&self.table[s..e]));
            }
            fmtd.finish()
        }
    }

    let dfa = DummyDFA {
        num_byte_classes: 2,
        table: vec![1, 2, 3, 4],
    };
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{ \"0\": [1], \"1\": [2], \"2\": [3], \"3\": [4] }");
}

