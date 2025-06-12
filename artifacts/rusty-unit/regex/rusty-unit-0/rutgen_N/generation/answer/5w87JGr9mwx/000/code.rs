// Answer 0

#[test]
fn test_fill_split_success() {
    struct TestInstPtr;
    struct TestSplit;
    
    enum MaybeInst {
        Split,
        Compiled(Inst),
    }
    
    enum Inst {
        Split(InstSplit),
    }
    
    struct InstSplit {
        goto1: TestInstPtr,
        goto2: TestInstPtr,
    }
    
    impl MaybeInst {
        fn fill_split(&mut self, goto1: TestInstPtr, goto2: TestInstPtr) {
            let filled = match *self {
                MaybeInst::Split => {
                    Inst::Split(InstSplit { goto1, goto2 })
                }
                _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", self),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let mut instruction = MaybeInst::Split;
    let ptr1 = TestInstPtr;
    let ptr2 = TestInstPtr;
    
    instruction.fill_split(ptr1, ptr2);
    
    match instruction {
        MaybeInst::Compiled(Inst::Split(_)) => {}
        _ => panic!("Expected Compiled with Split, but got: {:?}", instruction),
    }
}

#[test]
#[should_panic(expected = "must be called on Split instruction")]
fn test_fill_split_unreachable() {
    struct TestInstPtr;
    struct TestSplit;
    
    enum MaybeInst {
        Split,
        Compiled(Inst),
    }
    
    enum Inst {
        Split(InstSplit),
    }
    
    struct InstSplit {
        goto1: TestInstPtr,
        goto2: TestInstPtr,
    }
    
    impl MaybeInst {
        fn fill_split(&mut self, goto1: TestInstPtr, goto2: TestInstPtr) {
            let filled = match *self {
                MaybeInst::Split => {
                    Inst::Split(InstSplit { goto1, goto2 })
                }
                _ => unreachable!("must be called on Split instruction, instead it was called on: {:?}", self),
            };
            *self = MaybeInst::Compiled(filled);
        }
    }

    let mut instruction = MaybeInst::Compiled(Inst::Split(InstSplit { goto1: TestInstPtr, goto2: TestInstPtr }));
    let ptr1 = TestInstPtr;
    let ptr2 = TestInstPtr;
    
    instruction.fill_split(ptr1, ptr2);
}

