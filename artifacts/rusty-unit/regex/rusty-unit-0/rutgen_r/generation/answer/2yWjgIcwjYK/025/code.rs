// Answer 0

#[test]
fn test_fmt_with_split_inst_panics() {
    struct TestInst {
        goto1: usize,
        goto2: usize,
    }

    struct TestRegex {
        start: usize,
        inst: Vec<TestInst>,
    }

    impl TestRegex {
        fn iter(&self) -> std::slice::Iter<TestInst> {
            self.inst.iter()
        }
    }

    use std::fmt;

    impl fmt::Display for TestRegex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                match inst {
                    inst if pc != self.start => {
                        write!(f, "{:04} Split({}, {})", pc, inst.goto1, inst.goto2)?;
                    }
                    _ => {}
                }

                if pc == self.start {
                    write!(f, " (start)")?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    let regex = TestRegex {
        start: 0,
        inst: vec![
            TestInst { goto1: 1, goto2: 2 },
            TestInst { goto1: 3, goto2: 4 },
        ],
    };

    let result = format!("{}", regex);
    assert_eq!(result, "0000 Split(3, 4)\n0001 (start)\n");
}

#[test]
#[should_panic]
fn test_fmt_split_inst_with_err() {
    struct ErrorInst {
        goto1: usize,
        goto2: usize,
    }

    struct ErrorRegex {
        start: usize,
        inst: Vec<ErrorInst>,
    }

    impl ErrorRegex {
        fn iter(&self) -> std::slice::Iter<ErrorInst> {
            self.inst.iter()
        }
    }

    use std::fmt;

    impl fmt::Display for ErrorRegex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                match inst {
                    _ => {
                        return Err(fmt::Error);
                    }
                }
            }
            Ok(())
        }
    }

    let err_regex = ErrorRegex {
        start: 0,
        inst: vec![
            ErrorInst { goto1: 1, goto2: 2 },
        ],
    };

    let _ = format!("{}", err_regex);
}

