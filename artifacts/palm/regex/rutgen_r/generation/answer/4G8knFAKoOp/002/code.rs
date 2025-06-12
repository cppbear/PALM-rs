// Answer 0

#[derive(Debug)]
enum Inst {
    Save { goto: usize },
    Other,
}

struct Program {
    instructions: Vec<Inst>,
}

impl Program {
    fn new(instructions: Vec<Inst>) -> Self {
        Self { instructions }
    }

    fn skip(&self, mut pc: usize) -> usize {
        loop {
            match &self.instructions[pc] {
                Inst::Save { goto } => pc = *goto,
                _ => return pc,
            }
        }
    }
}

#[test]
fn test_skip_with_multiple_no_op_instructions() {
    let program = Program::new(vec![
        Inst::Save { goto: 2 },
        Inst::Save { goto: 4 },
        Inst::Other,
        Inst::Save { goto: 6 },
        Inst::Other,
        Inst::Other,
        Inst::Save { goto: 8 },
        Inst::Other,
    ]);
    assert_eq!(program.skip(0), 3);
}

#[test]
fn test_skip_with_single_no_op_instruction() {
    let program = Program::new(vec![
        Inst::Save { goto: 1 },
        Inst::Other,
    ]);
    assert_eq!(program.skip(0), 1);
}

#[test]
fn test_skip_with_no_no_op_instructions() {
    let program = Program::new(vec![
        Inst::Other,
        Inst::Other,
    ]);
    assert_eq!(program.skip(0), 0);
}

#[test]
fn test_skip_with_no_op_instruction_at_the_end() {
    let program = Program::new(vec![
        Inst::Save { goto: 1 },
        Inst::Other,
        Inst::Save { goto: 3 },
        Inst::Other,
        Inst::Other,
    ]);
    assert_eq!(program.skip(0), 3);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_skip_with_out_of_bounds_index() {
    let program = Program::new(vec![
        Inst::Save { goto: 0 },
    ]);
    program.skip(1);
}

