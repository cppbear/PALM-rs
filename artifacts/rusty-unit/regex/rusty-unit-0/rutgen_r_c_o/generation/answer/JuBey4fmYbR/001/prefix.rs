// Answer 0

#[test]
fn test_unwrap_uncompiled_inst_hole() {
    let inst_hole = InstHole::Save { slot: 1 };
    let maybe_inst = MaybeInst::Uncompiled(inst_hole);
    maybe_inst.unwrap();
}

#[test]
fn test_unwrap_split() {
    let split_inst = InstSplit { goto1: InstPtr(0), goto2: InstPtr(1) };
    let maybe_inst = MaybeInst::Split(split_inst);
    maybe_inst.unwrap();
}

#[test]
fn test_unwrap_split1() {
    let split_ptr = InstPtr(2);
    let maybe_inst = MaybeInst::Split1(split_ptr);
    maybe_inst.unwrap();
}

#[test]
fn test_unwrap_split2() {
    let split_ptr = InstPtr(3);
    let maybe_inst = MaybeInst::Split2(split_ptr);
    maybe_inst.unwrap();
}

