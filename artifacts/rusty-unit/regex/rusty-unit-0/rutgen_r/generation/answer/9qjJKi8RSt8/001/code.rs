// Answer 0

fn fill_test() {
    struct Inst {
        filled: bool,
    }

    impl Inst {
        fn fill(&mut self, _: InstPtr) {
            self.filled = true;
        }
    }

    struct Machine {
        insts: Vec<Inst>,
    }

    impl Machine {
        fn new(size: usize) -> Self {
            Self {
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

    #[derive(Clone)]
    enum Hole {
        None,
        One(usize),
        Many(Vec<Hole>),
    }

    type InstPtr = usize;

    // Test case 1: Hole::Many with Hole::One inside
    let mut machine = Machine::new(2);
    let hole_one = Hole::One(1);
    let holes = vec![hole_one.clone()];
    let hole_many = Hole::Many(holes.clone());

    machine.fill(hole_many.clone(), 0);
    assert!(machine.insts[1].filled);

    // Test case 2: Hole::Many with two Hole::One inside
    let hole_one2 = Hole::One(0);
    let holes2 = vec![hole_one.clone(), hole_one2.clone()];
    let hole_many2 = Hole::Many(holes2);

    let mut machine2 = Machine::new(2);
    machine2.fill(hole_many2.clone(), 0);
    assert!(machine2.insts[1].filled);
    assert!(machine2.insts[0].filled);

    // Test case 3: Hole::Many with an empty vector
    let hole_empty = Hole::Many(vec![]);
    let mut machine3 = Machine::new(2);
    machine3.fill(hole_empty, 0);
    assert!(!machine3.insts[0].filled);
    assert!(!machine3.insts[1].filled);

    // Test case 4: Hole::Many with Hole::None
    let hole_none = Hole::Many(vec![Hole::None]);
    let mut machine4 = Machine::new(2);
    machine4.fill(hole_none, 0);
    assert!(!machine4.insts[0].filled);
    assert!(!machine4.insts[1].filled);
}

