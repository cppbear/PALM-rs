// Answer 0

#[test]
fn test_fmt_with_save_inst() {
    use std::fmt;
    
    struct SaveInst {
        slot: usize,
        goto: usize,
    }

    struct Inst {
        inst: Vec<InstData>,
        start: usize,
    }

    enum InstData {
        Save(SaveInst),
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<InstData> {
            self.inst.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Function contents from the original code
            use self::InstData::*;

            fn with_goto(cur: usize, goto: usize, fmtd: String) -> String {
                if goto == cur + 1 {
                    fmtd
                } else {
                    format!("{} (goto: {})", fmtd, goto)
                }
            }

            for (pc, inst) in self.iter().enumerate() {
                match *inst {
                    Save(ref inst) => {
                        let s = format!("{:04} Save({})", pc, inst.slot);
                        write!(f, "{}", with_goto(pc, inst.goto, s))?;
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

    let saved_inst = SaveInst { slot: 42, goto: 1 };
    let instructions = vec![InstData::Save(saved_inst)];
    let instance = Inst { inst: instructions, start: 0 };
    
    let mut output = String::new();
    let result = write!(&mut output, "{}", instance);
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Write failed")]
fn test_fmt_should_panic_on_write_error() {
    use std::fmt;

    struct SaveInst {
        slot: usize,
        goto: usize,
    }

    struct Inst {
        inst: Vec<InstData>,
        start: usize,
    }

    enum InstData {
        Save(SaveInst),
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<InstData> {
            self.inst.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Function contents from the original code
            use self::InstData::*;

            fn with_goto(cur: usize, goto: usize, fmtd: String) -> String {
                if goto == cur + 1 {
                    fmtd
                } else {
                    format!("{} (goto: {})", fmtd, goto)
                }
            }

            for (pc, inst) in self.iter().enumerate() {
                match *inst {
                    Save(ref inst) => {
                        let s = format!("{:04} Save({})", pc, inst.slot);
                        // Force a panic on the first write to simulate an error
                        if pc == 0 { return Err(fmt::Error); }
                        write!(f, "{}", with_goto(pc, inst.goto, s))?;
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

    let saved_inst = SaveInst { slot: 42, goto: 1 };
    let instructions = vec![InstData::Save(saved_inst)];
    let instance = Inst { inst: instructions, start: 0 };
    
    let _ = write!(&mut String::new(), "{}", instance);
}

