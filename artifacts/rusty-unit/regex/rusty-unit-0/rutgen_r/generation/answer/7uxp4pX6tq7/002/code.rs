// Answer 0

#[derive(Debug)]
enum MaybeInst {
    Uncompiled(Inst),
    Split1(InstPtr),
    Split2(InstPtr),
    Compiled(Inst),
}

#[derive(Debug)]
struct InstSplit {
    goto1: InstPtr,
    goto2: InstPtr,
}

#[derive(Debug)]
struct Inst {
    split: Option<InstSplit>,
}

type InstPtr = usize;

impl Inst {
    fn fill(&self, goto: InstPtr) -> Inst {
        Inst {
            split: self.split.clone(),
        }
    }
}

#[test]
fn test_fill_split2() {
    let mut maybe_inst = MaybeInst::Split2(2);
    let goto = 3;

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst { split: Some(InstSplit { goto1, goto2 }) }) = maybe_inst {
        assert_eq!(goto1, goto);
        assert_eq!(goto2, 2);
    } else {
        panic!("Expected MaybeInst::Compiled with filled instruction");
    }
}

#[test]
fn test_fill_split1() {
    let mut maybe_inst = MaybeInst::Split1(1);
    let goto = 4;

    maybe_inst.fill(goto);

    if let MaybeInst::Compiled(Inst { split: Some(InstSplit { goto1, goto2 }) }) = maybe_inst {
        assert_eq!(goto1, 1);
        assert_eq!(goto2, goto);
    } else {
        panic!("Expected MaybeInst::Compiled with filled instruction");
    }
}

