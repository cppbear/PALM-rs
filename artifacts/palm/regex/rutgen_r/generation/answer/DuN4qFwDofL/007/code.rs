// Answer 0

#[derive(Default)]
struct Inst {
    // Mock functions based on the actual implementations
}

impl Inst {
    fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {
        // Mock behavior for fill_split
    }

    fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {
        // Mock behavior for half_fill_split_goto1
    }

    fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {
        // Mock behavior for half_fill_split_goto2
    }
}

#[derive(Debug)]
enum Hole {
    None,
    One(usize),
    Many(Vec<Hole>),
}

struct InstPtr;

struct TestStruct {
    insts: Vec<Inst>,
}

impl TestStruct {
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
fn test_fill_split_with_both_gotos() {
    let mut test_struct = TestStruct { insts: vec![Inst::default(); 10] };
    let hole = Hole::One(0);
    let goto1 = Some(InstPtr);
    let goto2 = Some(InstPtr);

    let result = test_struct.fill_split(hole, goto1, goto2);
    assert_eq!(result, Hole::None);
}

