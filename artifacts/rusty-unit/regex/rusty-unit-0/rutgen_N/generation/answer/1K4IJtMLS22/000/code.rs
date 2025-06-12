// Answer 0

#[test]
fn test_push_hole() {
    struct TestStruct {
        insts: Vec<MaybeInst>,
    }

    impl TestStruct {
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    enum MaybeInst {
        Uncompiled(InstHole),
    }

    struct InstHole;

    enum Hole {
        One(usize),
    }

    let mut test_struct = TestStruct { insts: Vec::new() };
    let inst = InstHole;

    let hole_result = test_struct.push_hole(inst);
    match hole_result {
        Hole::One(index) => assert_eq!(index, 0),
    }

    assert_eq!(test_struct.insts.len(), 1);
}

#[test]
fn test_push_hole_multiple() {
    struct TestStruct {
        insts: Vec<MaybeInst>,
    }

    impl TestStruct {
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }

    enum MaybeInst {
        Uncompiled(InstHole),
    }

    struct InstHole;

    enum Hole {
        One(usize),
    }

    let mut test_struct = TestStruct { insts: Vec::new() };

    for _ in 0..5 {
        let inst = InstHole;
        let hole_result = test_struct.push_hole(inst);
        match hole_result {
            Hole::One(index) => assert_eq!(index, test_struct.insts.len() - 1),
        }
    }

    assert_eq!(test_struct.insts.len(), 5);
}

