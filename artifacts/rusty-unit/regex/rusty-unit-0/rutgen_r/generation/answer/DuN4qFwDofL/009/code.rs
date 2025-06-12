// Answer 0

#[test]
fn test_fill_split_with_hole_one_and_goto2() {
    struct Inst {
        filled: bool,
    }

    impl Inst {
        fn half_fill_split_goto2(&mut self, _goto2: usize) {
            self.filled = true;
        }
    }

    struct Machine {
        insts: Vec<Inst>,
    }

    enum Hole {
        None,
        One(usize),
        Many(Vec<Hole>),
    }

    impl Machine {
        fn fill_split(
            &mut self,
            hole: Hole,
            goto1: Option<usize>,
            goto2: Option<usize>,
        ) -> Hole {
            match hole {
                Hole::None => Hole::None,
                Hole::One(pc) => {
                    match (goto1, goto2) {
                        (Some(goto1), Some(goto2)) => {
                            self.insts[pc].half_fill_split_goto2(goto2);
                            Hole::None
                        }
                        (Some(goto1), None) => {
                            self.insts[pc].half_fill_split_goto2(goto1);
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

    let pc = 0;
    let mut machine = Machine { insts: vec![Inst { filled: false }] };

    let result = machine.fill_split(Hole::One(pc), None, Some(1));
    assert_eq!(result, Hole::One(pc));
    assert!(machine.insts[pc].filled);
}

