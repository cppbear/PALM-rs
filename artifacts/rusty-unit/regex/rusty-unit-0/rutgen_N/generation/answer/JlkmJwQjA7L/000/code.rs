// Answer 0

#[test]
fn test_fmt_with_no_states() {
    struct TestDFA(Vec<u32>); // Example struct

    impl std::fmt::Debug for TestDFA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut fmtd = f.debug_map();
            for (b, si) in self.0.iter().enumerate() {
                match *si {
                    0 => {}
                    1 => {
                        fmtd.entry(&b, &"DEAD");
                    }
                    si => {
                        fmtd.entry(&b, &si.to_string());
                    }
                }
            }
            fmtd.finish()
        }
    }

    let dfa = TestDFA(vec![]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{}");
}

#[test]
fn test_fmt_with_dead_state() {
    struct TestDFA(Vec<u32>); // Example struct

    impl std::fmt::Debug for TestDFA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut fmtd = f.debug_map();
            for (b, si) in self.0.iter().enumerate() {
                match *si {
                    0 => {}
                    1 => {
                        fmtd.entry(&b, &"DEAD");
                    }
                    si => {
                        fmtd.entry(&b, &si.to_string());
                    }
                }
            }
            fmtd.finish()
        }
    }

    let dfa = TestDFA(vec![1]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{0: \"DEAD\"}");
}

#[test]
fn test_fmt_with_various_states() {
    struct TestDFA(Vec<u32>); // Example struct

    impl std::fmt::Debug for TestDFA {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut fmtd = f.debug_map();
            for (b, si) in self.0.iter().enumerate() {
                match *si {
                    0 => {}
                    1 => {
                        fmtd.entry(&b, &"DEAD");
                    }
                    si => {
                        fmtd.entry(&b, &si.to_string());
                    }
                }
            }
            fmtd.finish()
        }
    }

    let dfa = TestDFA(vec![0, 2, 1, 3]);
    let result = format!("{:?}", dfa);
    assert_eq!(result, "{1: \"DEAD\", 2: \"2\", 3: \"3\"}");
}

