// Answer 0

#[test]
fn test_fill_split_when_self_is_not_split() {
    let mut instance = MaybeInst::Compiled(Inst::Match(0));
    let goto1 = 10;
    let goto2 = 20;
    instance.fill_split(goto1, goto2);
}

#[test]
fn test_fill_split_when_self_is_not_split_edge_case() {
    let mut instance = MaybeInst::Uncompiled(InstHole::Char { c: 'a' });
    let goto1 = 1;
    let goto2 = 2;
    instance.fill_split(goto1, goto2);
}

#[test]
fn test_fill_split_with_high_values() {
    let mut instance = MaybeInst::Compiled(Inst::Save(InstSave::new()));
    let goto1 = 999;
    let goto2 = 1000;
    instance.fill_split(goto1, goto2);
}

#[test]
#[should_panic]
fn test_fill_split_when_self_is_split() {
    let mut instance = MaybeInst::Split;
    let goto1 = 5;
    let goto2 = 15;
    instance.fill_split(goto1, goto2);
}

