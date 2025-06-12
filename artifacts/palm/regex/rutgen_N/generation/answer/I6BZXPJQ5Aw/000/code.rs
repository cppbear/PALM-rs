// Answer 0

#[derive(Debug)]
struct InstHole {
    look: EmptyLook,
}

struct Patch {
    hole: usize,
    entry: usize,
}

struct EmptyLook;

struct Compiler {
    insts: Vec<InstHole>,
}

impl Compiler {
    fn new() -> Self {
        Compiler { insts: Vec::new() }
    }

    fn push_hole(&mut self, hole: InstHole) -> usize {
        self.insts.push(hole);
        self.insts.len() - 1
    }

    fn c_empty_look(&mut self, look: EmptyLook) -> Result<Patch, String> {
        let hole = self.push_hole(InstHole { look });
        Ok(Patch { hole, entry: self.insts.len() - 1 })
    }
}

#[test]
fn test_c_empty_look() {
    let mut compiler = Compiler::new();
    let empty_look = EmptyLook;

    let result = compiler.c_empty_look(empty_look).expect("Expected a valid result");

    assert_eq!(result.hole, 0);
    assert_eq!(result.entry, 0);
}

#[test]
fn test_c_empty_look_multiple_calls() {
    let mut compiler = Compiler::new();
    
    let first_look = EmptyLook;
    let second_look = EmptyLook;

    let first_result = compiler.c_empty_look(first_look).expect("Expected a valid result");
    let second_result = compiler.c_empty_look(second_look).expect("Expected a valid result");

    assert_eq!(first_result.hole, 0);
    assert_eq!(first_result.entry, 0);
    assert_eq!(second_result.hole, 1);
    assert_eq!(second_result.entry, 1);
}

