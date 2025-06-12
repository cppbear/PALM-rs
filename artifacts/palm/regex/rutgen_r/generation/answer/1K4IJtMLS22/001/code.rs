// Answer 0

#[test]
fn test_push_hole_creates_hole_successfully() {
    struct InstHole;
    struct Compiler {
        insts: Vec<MaybeInst>,
    }
    
    enum MaybeInst {
        Uncompiled(InstHole),
    }
    
    enum Hole {
        One(usize),
    }
    
    impl Compiler {
        fn new() -> Self {
            Compiler {
                insts: Vec::new(),
            }
        }
        
        fn push_hole(&mut self, inst: InstHole) -> Hole {
            let hole = self.insts.len();
            self.insts.push(MaybeInst::Uncompiled(inst));
            Hole::One(hole)
        }
    }
    
    let mut compiler = Compiler::new();
    let inst = InstHole;
    let result = compiler.push_hole(inst);
    match result {
        Hole::One(hole) => assert_eq!(hole, 0),
        _ => panic!("Expected Hole::One variant"),
    }
    
    let another_hole = compiler.push_hole(InstHole);
    match another_hole {
        Hole::One(hole) => assert_eq!(hole, 1),
        _ => panic!("Expected Hole::One variant"),
    }
}

