// Answer 0

#[test]
fn test_can_exec_dfa_size_limit_zero() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program { dfa_size_limit, instructions }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Char(char),
            Ranges(Vec<(char, char)>),
            EmptyLook(usize),
            Match(usize),
            Save(usize),
            Split(usize, usize),
            Bytes(Vec<u8>),
        }
    }

    let insts = Program::new(0, vec![
        prog::Inst::Save(1),
        prog::Inst::EmptyLook(2),
        prog::Inst::Match(3),
    ]);

    assert_eq!(can_exec(&insts), false);
}

#[test]
fn test_can_exec_len_exceeding_i32_max() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program { dfa_size_limit, instructions }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Char(char),
            Ranges(Vec<(char, char)>),
            EmptyLook(usize),
            Match(usize),
            Save(usize),
            Split(usize, usize),
            Bytes(Vec<u8>),
        }
    }

    let insts = Program::new(1, vec![prog::Inst::Save(1); ::std::i32::MAX as usize + 1]);

    assert_eq!(can_exec(&insts), false);
}

#[test]
fn test_can_exec_with_valid_instructions() {
    struct Program {
        dfa_size_limit: usize,
        instructions: Vec<prog::Inst>,
    }

    impl Program {
        fn new(dfa_size_limit: usize, instructions: Vec<prog::Inst>) -> Self {
            Program { dfa_size_limit, instructions }
        }

        fn len(&self) -> usize {
            self.instructions.len()
        }
    }

    mod prog {
        #[derive(Clone)]
        pub enum Inst {
            Char(char),
            Ranges(Vec<(char, char)>),
            EmptyLook(usize),
            Match(usize),
            Save(usize),
            Split(usize, usize),
            Bytes(Vec<u8>),
        }
    }

    let insts = Program::new(1, vec![
        prog::Inst::Save(1),
        prog::Inst::EmptyLook(2),
        prog::Inst::Match(3),
    ]);

    assert_eq!(can_exec(&insts), true);
}

