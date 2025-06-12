// Answer 0

#[test]
fn test_fmt_with_char_inst() {
    use std::fmt;

    #[derive(Debug)]
    struct CharInst {
        c: char,
        goto: usize,
    }

    #[derive(Debug)]
    struct Inst {
        inst: Vec<InstEnum>,
        start: usize,
    }

    #[derive(Debug)]
    enum InstEnum {
        Char(CharInst),
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<InstEnum> {
            self.inst.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                match inst {
                    InstEnum::Char(inst) => {
                        let s = format!("{:?}", inst.c);
                        write!(f, "{:04} {}", pc, inst.goto)?;
                    }
                }
                if pc == self.start {
                    write!(f, " (start)")?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    let char_inst = CharInst { c: 'a', goto: 0 };
    let instructions = vec![InstEnum::Char(char_inst)];
    let inst = Inst { inst: instructions, start: 0 };

    // Capture the output
    let result = format!("{}", inst);
    assert_eq!(result, "0000 0\n (start)\n");
}

#[test]
#[should_panic(expected = "Write failed")]
fn test_fmt_with_char_inst_panic() {
    use std::fmt;

    #[derive(Debug)]
    struct CharInst {
        c: char,
        goto: usize,
    }

    #[derive(Debug)]
    struct Inst {
        inst: Vec<InstEnum>,
        start: usize,
    }

    #[derive(Debug)]
    enum InstEnum {
        Char(CharInst),
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<InstEnum> {
            self.inst.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for (pc, inst) in self.iter().enumerate() {
                match inst {
                    InstEnum::Char(inst) => {
                        // Err one condition to trigger a panic
                        if inst.goto == 0 {
                            return Err(fmt::Error);
                        }
                        write!(f, "{:04} {}", pc, inst.goto)?;
                    }
                }
                if pc == self.start {
                    write!(f, " (start)")?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    let char_inst = CharInst { c: 'b', goto: 0 }; // This should cause a panic
    let instructions = vec![InstEnum::Char(char_inst)];
    let inst = Inst { inst: instructions, start: 0 };

    // This should panic due to the Error condition
    let _ = format!("{}", inst);
}

