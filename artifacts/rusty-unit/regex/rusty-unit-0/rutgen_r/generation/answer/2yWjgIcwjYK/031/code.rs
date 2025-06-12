// Answer 0

fn test_fmt_with_save_inst_panics() {
    struct SaveInst {
        slot: usize,
        goto: usize,
    }

    struct Inst {
        inst: InstEnum,
    }

    enum InstEnum {
        Save(SaveInst),
    }

    struct Insts {
        instructions: Vec<Inst>,
        start: usize,
    }

    impl Insts {
        fn iter(&self) -> std::slice::Iter<Inst> {
            self.instructions.iter()
        }
    }

    use std::fmt;

    struct FmtTest {
        insts: Insts,
    }

    impl fmt::Display for FmtTest {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.insts.fmt(f)
        }
    }

    let insts = Insts {
        instructions: vec![Inst { inst: InstEnum::Save(SaveInst { slot: 0, goto: 0 }) }],
        start: 0,
    };

    let fmt_test = FmtTest { insts };
    
    // Will panic if the gto is 0 due to the logic being invoked
    let result = std::panic::catch_unwind(|| {
        let _ = fmt::format(fmt_test);
    });

    assert!(result.is_err());
}

fn test_fmt_with_save_inst_no_panic() {
    struct SaveInst {
        slot: usize,
        goto: usize,
    }

    struct Inst {
        inst: InstEnum,
    }

    enum InstEnum {
        Save(SaveInst),
    }

    struct Insts {
        instructions: Vec<Inst>,
        start: usize,
    }

    impl Insts {
        fn iter(&self) -> std::slice::Iter<Inst> {
            self.instructions.iter()
        }
    }

    use std::fmt;

    struct FmtTest {
        insts: Insts,
    }

    impl fmt::Display for FmtTest {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.insts.fmt(f)
        }
    }

    let insts = Insts {
        instructions: vec![
            Inst { inst: InstEnum::Save(SaveInst { slot: 1, goto: 2 }) },
            Inst { inst: InstEnum::Save(SaveInst { slot: 2, goto: 3 }) },
        ],
        start: 0,
    };

    let fmt_test = FmtTest { insts };
    
    let result = fmt::format(fmt_test);
    
    assert_eq!(result, "0000 Save(1) (goto: 2)\n0001 Save(2)\n");
}

