// Answer 0

#[derive(Debug)]
struct Inst {
    filled: bool,
}

impl Inst {
    fn fill(&mut self, _goto: InstPtr) {
        self.filled = true;
    }
}

#[derive(Debug)]
struct InstPtr(usize);

#[derive(Debug)]
enum Hole {
    None,
    One(usize),
    Many(Vec<Hole>),
}

struct Compiler {
    insts: Vec<Inst>,
}

impl Compiler {
    fn new(size: usize) -> Self {
        Compiler {
            insts: vec![Inst { filled: false }; size],
        }
    }

    fn fill(&mut self, hole: Hole, goto: InstPtr) {
        match hole {
            Hole::None => {}
            Hole::One(pc) => {
                self.insts[pc].fill(goto);
            }
            Hole::Many(holes) => {
                for hole in holes {
                    self.fill(hole, goto);
                }
            }
        }
    }
}

#[test]
fn test_fill_with_hole_many() {
    let goto = InstPtr(0);
    let mut compiler = Compiler::new(5);

    let holes = vec![
        Hole::One(2), // this should fill insts[2]
        Hole::Many(vec![Hole::One(3), Hole::One(4)]), // this should fill insts[3] and insts[4]
    ];

    let hole = Hole::Many(holes);
    compiler.fill(hole, goto);

    assert_eq!(compiler.insts[2].filled, true);
    assert_eq!(compiler.insts[3].filled, true);
    assert_eq!(compiler.insts[4].filled, true);
}

#[test]
fn test_fill_with_hole_many_empty() {
    let goto = InstPtr(1);
    let mut compiler = Compiler::new(5);

    let hole = Hole::Many(vec![]); // an empty Hole::Many
    compiler.fill(hole, goto);

    for inst in compiler.insts.iter() {
        assert_eq!(inst.filled, false); // no inst should be filled
    }
}

