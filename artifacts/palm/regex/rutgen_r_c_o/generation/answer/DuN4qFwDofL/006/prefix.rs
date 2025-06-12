// Answer 0

#[test]
fn test_fill_split_with_many_holes_and_non_empty_holes() {
    struct DummyInst;
    
    impl DummyInst {
        fn new() -> Self {
            DummyInst {}
        }
        
        fn fill_split(&mut self, _goto1: InstPtr, _goto2: InstPtr) {}
        fn half_fill_split_goto1(&mut self, _goto1: InstPtr) {}
        fn half_fill_split_goto2(&mut self, _goto2: InstPtr) {}
    }

    let mut compiler = Compiler::new();
    compiler.insts.push(MaybeInst::Compiled(DummyInst::new()));
    
    let hole1 = Hole::None;
    let hole2 = Hole::None;
    let hole = Hole::Many(vec![hole1, hole2]);
    let goto1 = Some(1);
    let goto2 = Some(2);

    let result = compiler.fill_split(hole, goto1, goto2);
}

