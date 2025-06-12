// Answer 0

#[test]
fn test_half_fill_split_goto2_with_compiled_inst() {
    let mut inst = MaybeInst::Compiled(Inst::Match(0));
    let goto2 = InstPtr::from(1);
    inst.half_fill_split_goto2(goto2);
}

#[test]
fn test_half_fill_split_goto2_with_uncompiled_inst() {
    let mut inst = MaybeInst::Uncompiled(InstHole::Save { slot: 0 });
    let goto2 = InstPtr::from(2);
    inst.half_fill_split_goto2(goto2);
}

#[test]
fn test_half_fill_split_goto2_with_split1_inst() {
    let mut inst = MaybeInst::Split1(InstPtr::from(3));
    let goto2 = InstPtr::from(4);
    inst.half_fill_split_goto2(goto2);
}

#[test]
fn test_half_fill_split_goto2_with_split2_inst() {
    let mut inst = MaybeInst::Split2(InstPtr::from(5));
    let goto2 = InstPtr::from(0);
    inst.half_fill_split_goto2(goto2);
}

