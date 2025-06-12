// Answer 0

fn test_fmt_with_match() {
    use std::fmt;

    struct Inst {
        slot: usize,
    }

    struct TestStruct {
        inst: Vec<Inst>,
        start: usize,
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<Inst> {
            self.inst.iter()
        }
    }

    impl fmt::Debug for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.slot)
        }
    }

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                write!(f, "{:04} Match({:?})\n", pc, inst)?;
            }
            Ok(())
        }
    }

    let test_struct = TestStruct {
        inst: vec![Inst { slot: 0 }, Inst { slot: 1 }],
        start: 0,
    };

    let result = format!("{}", test_struct);
    assert_eq!(result, "0000 Match(0)\n0001 Match(1)\n");
}

fn test_fmt_with_no_start() {
    use std::fmt;

    struct Inst {
        slot: usize,
    }

    struct TestStruct {
        inst: Vec<Inst>,
        start: usize,
    }

    impl TestStruct {
        fn iter(&self) -> std::slice::Iter<Inst> {
            self.inst.iter()
        }
    }

    impl fmt::Debug for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.slot)
        }
    }

    impl fmt::Display for TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                let initial = write!(f, "{:04} Match({:?})", pc, inst)?;
                if pc != self.start {
                    f.write_str(" (not start)")?;
                }
                initial?;
            }
            Ok(())
        }
    }

    let test_struct = TestStruct {
        inst: vec![Inst { slot: 0 }, Inst { slot: 1 }],
        start: 1,
    };

    let result = format!("{}", test_struct);
    assert_eq!(result, "0000 Match(0) (not start)0001 Match(1) (not start)");
}

