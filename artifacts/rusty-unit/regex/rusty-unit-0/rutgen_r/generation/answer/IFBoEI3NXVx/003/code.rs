// Answer 0

#[test]
fn test_can_exec_valid_dfa() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program {
                dfa_size_limit,
                instructions,
            }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Bytes(Vec<u8>),
            EmptyLook(u32),
            Match(u32),
            Save(u32),
            Split(u32, u32),
            Char(char),
            Ranges(Vec<(char, char)>),
        }
    }

    let insts = Program::new(1, vec![
        prog::Inst::Bytes(vec![1, 2, 3]),
        prog::Inst::EmptyLook(0),
        prog::Inst::Match(0),
        prog::Inst::Save(0),
        prog::Inst::Split(0, 1),
    ]);

    assert_eq!(can_exec(&insts), true);
}

#[test]
fn test_can_exec_exceed_max_instructions() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program {
                dfa_size_limit,
                instructions,
            }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Bytes(Vec<u8>),
            EmptyLook(u32),
            Match(u32),
            Save(u32),
            Split(u32, u32),
            Char(char),
            Ranges(Vec<(char, char)>),
        }
    }

    let insts = Program::new(1, vec![prog::Inst::Bytes(vec![1, 2, 3]); ::std::i32::MAX as usize + 1]);

    assert_eq!(can_exec(&insts), false);
}

#[test]
fn test_can_exec_zero_dfa_size_limit() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program {
                dfa_size_limit,
                instructions,
            }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Bytes(Vec<u8>),
            EmptyLook(u32),
            Match(u32),
            Save(u32),
            Split(u32, u32),
            Char(char),
            Ranges(Vec<(char, char)>),
        }
    }

    let insts = Program::new(0, vec![
        prog::Inst::Bytes(vec![1, 2, 3]),
        prog::Inst::EmptyLook(0),
    ]);

    assert_eq!(can_exec(&insts), false);
}

#[test]
fn test_can_exec_unicode_instruction() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program {
                dfa_size_limit,
                instructions,
            }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Bytes(Vec<u8>),
            EmptyLook(u32),
            Match(u32),
            Save(u32),
            Split(u32, u32),
            Char(char),
            Ranges(Vec<(char, char)>),
        }
    }

    let insts = Program::new(1, vec![prog::Inst::Char('a')]);

    assert_eq!(can_exec(&insts), false);
}

