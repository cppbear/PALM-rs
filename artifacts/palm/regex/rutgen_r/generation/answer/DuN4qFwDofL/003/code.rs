// Answer 0

#[derive(Debug)]
struct InstPtr;

#[derive(Debug)]
struct Instruction;

impl Instruction {
    fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
    fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
    fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
}

#[derive(Debug)]
enum Hole {
    None,
    One(usize),
    Many(Vec<Hole>),
}

struct Regex {
    insts: Vec<Instruction>,
}

impl Regex {
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
fn test_fill_split_many_holes() {
    let mut regex = Regex { insts: vec![Instruction] };
    
    let goto1 = Some(InstPtr);
    let goto2 = Some(InstPtr);
    
    let hole = Hole::Many(vec![
        Hole::One(0),
        Hole::One(0), // This should hit the case where both goto1 and goto2 are supplied
    ]);
    
    let result = regex.fill_split(hole, goto1.clone(), goto2.clone());
    
    if let Hole::Many(new_holes) = result {
        assert_eq!(new_holes.len(), 2); // expect two holes back
    } else {
        panic!("Expected Hole::Many but got something else");
    }
}

#[test]
fn test_fill_split_empty_new_holes() {
    let mut regex = Regex { insts: vec![Instruction] };

    let hole = Hole::Many(vec![]);
    
    let result = regex.fill_split(hole, None, None);
    assert_eq!(result, Hole::None); // expect Hole::None
}

#[test]
fn test_fill_split_single_new_hole() {
    let mut regex = Regex { insts: vec![Instruction] };
    
    let goto1 = Some(InstPtr);
    
    let hole = Hole::Many(vec![
        Hole::One(0),
    ]);

    let result = regex.fill_split(hole, goto1, None);
    
    assert_eq!(result, Hole::One(0)); // expect Hole::One
}

