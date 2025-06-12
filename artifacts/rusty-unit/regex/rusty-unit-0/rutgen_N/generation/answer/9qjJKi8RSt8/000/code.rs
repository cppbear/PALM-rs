// Answer 0

#[derive(Debug)]
enum Hole {
    None,
    One(usize),
    Many(Vec<Hole>),
}

#[derive(Debug)]
struct Instruction {
    filled: Option<InstPtr>,
}

impl Instruction {
    fn fill(&mut self, goto: InstPtr) {
        self.filled = Some(goto);
    }
}

type InstPtr = usize;

#[derive(Debug)]
struct Compiler {
    insts: Vec<Instruction>,
}

#[test]
fn test_fill_none() {
    let mut compiler = Compiler { insts: vec![] };
    compiler.fill(Hole::None, 0);
    assert!(compiler.insts.is_empty());
}

#[test]
fn test_fill_one() {
    let mut compiler = Compiler {
        insts: vec![Instruction { filled: None }],
    };
    compiler.fill(Hole::One(0), 1);
    assert_eq!(compiler.insts[0].filled, Some(1));
}

#[test]
fn test_fill_many() {
    let mut compiler = Compiler {
        insts: vec![Instruction { filled: None }, Instruction { filled: None }],
    };
    let holes = vec![Hole::One(0), Hole::One(1)];
    compiler.fill(Hole::Many(holes), 2);
    assert_eq!(compiler.insts[0].filled, Some(2));
    assert_eq!(compiler.insts[1].filled, Some(2));
}

#[test]
fn test_fill_nested_many() {
    let mut compiler = Compiler {
        insts: vec![Instruction { filled: None }, Instruction { filled: None }, Instruction { filled: None }],
    };
    let holes = vec![Hole::Many(vec![Hole::One(0), Hole::One(1)]), Hole::One(2)];
    compiler.fill(Hole::Many(holes), 3);
    assert_eq!(compiler.insts[0].filled, Some(3));
    assert_eq!(compiler.insts[1].filled, Some(3));
    assert_eq!(compiler.insts[2].filled, Some(3));
}

