// Answer 0

#[test]
fn test_unwrap_compiled_inst() {
    let inst = Inst::Match(0);
    let maybe_inst = MaybeInst::Compiled(inst.clone());
    assert_eq!(maybe_inst.unwrap(), inst);
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_uncompiled_inst() {
    let inst_hole = InstHole::Save { slot: 1 };
    let maybe_inst = MaybeInst::Uncompiled(inst_hole);
    maybe_inst.unwrap(); // This should panic
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_split() {
    let maybe_inst = MaybeInst::Split;
    maybe_inst.unwrap(); // This should panic
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_split1() {
    let ptr = InstPtr(1);
    let maybe_inst = MaybeInst::Split1(ptr);
    maybe_inst.unwrap(); // This should panic
}

#[test]
#[should_panic(expected = "must be called on a compiled instruction")]
fn test_unwrap_split2() {
    let ptr = InstPtr(2);
    let maybe_inst = MaybeInst::Split2(ptr);
    maybe_inst.unwrap(); // This should panic
}

