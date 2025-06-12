// Answer 0

#[test]
fn test_half_fill_split_goto1_with_compiled() {
    let mut instruction = MaybeInst::Compiled(Inst::Match(0));
    let goto = InstPtr::default();
    instruction.half_fill_split_goto1(goto);
}

#[test]
fn test_half_fill_split_goto1_with_uncompiled() {
    let mut instruction = MaybeInst::Uncompiled(InstHole::Save { slot: 1 });
    let goto = InstPtr::default();
    instruction.half_fill_split_goto1(goto);
}

#[test]
fn test_half_fill_split_goto1_with_split2() {
    let mut instruction = MaybeInst::Split2(InstPtr::default());
    let goto = InstPtr::default();
    instruction.half_fill_split_goto1(goto);
}

