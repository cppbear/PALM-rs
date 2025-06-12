// Answer 0

#[derive(Debug)]
struct Instruction;

impl Instruction {
    fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
    fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
    fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
}

#[derive(Debug)]
struct InstPtr;

#[derive(Debug)]
enum Hole {
    None,
    One(usize),
    Many(Vec<Hole>),
}

#[derive(Debug)]
struct Compiler {
    insts: Vec<Instruction>,
}

impl Compiler {
    fn new(size: usize) -> Self {
        Compiler {
            insts: vec![Instruction; size],
        }
    }

    fn fill_split(
        &mut self,
        hole: Hole,
        goto1: Option<InstPtr>,
        goto2: Option<InstPtr>,
    ) -> Hole {
        match hole {
            Hole::None => Hole::None,
            Hole::One(pc) => {
                match (goto1, goto2) {
                    (Some(goto1), Some(goto2)) => {
                        self.insts[pc].fill_split(goto1, goto2);
                        Hole::None
                    }
                    (Some(goto1), None) => {
                        self.insts[pc].half_fill_split_goto1(goto1);
                        Hole::One(pc)
                    }
                    (None, Some(goto2)) => {
                        self.insts[pc].half_fill_split_goto2(goto2);
                        Hole::One(pc)
                    }
                    (None, None) => unreachable!("at least one of the split holes must be filled"),
                }
            }
            Hole::Many(holes) => {
                let mut new_holes = vec![];
                for hole in holes {
                    new_holes.push(self.fill_split(hole, goto1, goto2));
                }
                if new_holes.is_empty() {
                    Hole::None
                } else if new_holes.len() == 1 {
                    new_holes.pop().unwrap()
                } else {
                    Hole::Many(new_holes)
                }
            }
        }
    }
}

#[test]
fn test_fill_split_none() {
    let mut compiler = Compiler::new(1);
    let result = compiler.fill_split(Hole::None, None, None);
    assert_eq!(result, Hole::None);
}

#[test]
fn test_fill_split_one_with_both() {
    let mut compiler = Compiler::new(1);
    let pc = 0;
    let hole = Hole::One(pc);
    let goto1 = Some(InstPtr);
    let goto2 = Some(InstPtr);
    let result = compiler.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None);
}

#[test]
fn test_fill_split_one_with_goto1() {
    let mut compiler = Compiler::new(1);
    let pc = 0;
    let hole = Hole::One(pc);
    let goto1 = Some(InstPtr);
    let result = compiler.fill_split(hole, goto1, None);
    assert_eq!(result, Hole::One(pc));
}

#[test]
fn test_fill_split_one_with_goto2() {
    let mut compiler = Compiler::new(1);
    let pc = 0;
    let hole = Hole::One(pc);
    let goto2 = Some(InstPtr);
    let result = compiler.fill_split(hole, None, goto2);
    assert_eq!(result, Hole::One(pc));
}

#[test]
fn test_fill_split_many_empty() {
    let mut compiler = Compiler::new(1);
    let hole = Hole::Many(vec![]);
    let result = compiler.fill_split(hole, None, None);
    assert_eq!(result, Hole::None);
}

#[test]
fn test_fill_split_many_one() {
    let mut compiler = Compiler::new(2);
    let hole = Hole::Many(vec![Hole::One(0)]);
    let goto1 = Some(InstPtr);
    let result = compiler.fill_split(hole, goto1, None);
    assert_eq!(result, Hole::One(0));
}

#[test]
fn test_fill_split_many() {
    let mut compiler = Compiler::new(2);
    let hole = Hole::Many(vec![Hole::One(0), Hole::One(1)]);
    let goto1 = Some(InstPtr);
    let goto2 = Some(InstPtr);
    let result = compiler.fill_split(hole, goto1, goto2);
    if let Hole::Many(ref new_holes) = result {
        assert_eq!(new_holes.len(), 2);
    } else {
        panic!("Expected Many variant");
    }
}

