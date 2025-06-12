// Answer 0

#[test]
fn test_fmt_split() {
    use std::fmt;

    #[derive(Debug)]
    struct Inst {
        start: usize,
        instructions: Vec<Instruction>,
    }

    #[derive(Debug)]
    struct Instruction {
        goto1: usize,
        goto2: usize,
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<Instruction> {
            self.instructions.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use self::Instruction::*;

            let pc = 0; // start pc
            for (idx, inst) in self.iter().enumerate() {
                match inst {
                    Instruction { goto1, goto2 } => {
                        write!(f, "{:04} Split({}, {})", pc, goto1, goto2)?;
                        if pc == self.start {
                            write!(f, " (start)")?;
                        }
                        write!(f, "\n")?;
                    }
                }
            }
            Ok(())
        }
    }

    let instructions = vec![Instruction { goto1: 1, goto2: 2 }];
    let inst = Inst { start: 0, instructions };

    let mut output = String::new();
    let result = write!(output, "{}", inst);

    assert!(result.is_ok());
    assert!(output.contains("0000 Split(1, 2)"));
    assert!(output.contains(" (start)"));
}

#[test]
#[should_panic]
fn test_fmt_split_panic() {
    use std::fmt;

    #[derive(Debug)]
    struct Inst {
        start: usize,
        instructions: Vec<Instruction>,
    }

    #[derive(Debug)]
    struct Instruction {
        goto1: usize,
        goto2: usize,
    }

    impl Inst {
        fn iter(&self) -> std::slice::Iter<Instruction> {
            self.instructions.iter()
        }
    }

    impl fmt::Display for Inst {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let pc = 0; // start pc
            for (idx, inst) in self.iter().enumerate() {
                match inst {
                    Instruction { goto1, goto2 } => {
                        // Simulate a panic situation
                        if goto1 % 2 == 0 {
                            panic!("Intentional panic for even goto1");
                        }
                        write!(f, "{:04} Split({}, {})", pc, goto1, goto2)?;
                        if pc == self.start {
                            write!(f, " (start)")?;
                        }
                        write!(f, "\n")?;
                    }
                }
            }
            Ok(())
        }
    }

    let instructions = vec![Instruction { goto1: 2, goto2: 3 }];
    let inst = Inst { start: 0, instructions };

    let _ = format!("{}", inst); //This should cause panic
}

