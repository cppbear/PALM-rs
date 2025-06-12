// Answer 0

#[test]
fn test_unwrap_match() {
    let inst = MaybeInst::Compiled(Inst::Match(0));
    inst.unwrap();
}

#[test]
fn test_unwrap_save() {
    let inst_save = InstSave { slot: 0 };
    let inst = MaybeInst::Compiled(Inst::Save(inst_save));
    inst.unwrap();
}

#[test]
fn test_unwrap_split() {
    let goto1 = InstPtr::default();
    let goto2 = InstPtr::default();
    let inst_split = InstSplit { goto1, goto2 };
    let inst = MaybeInst::Compiled(Inst::Split(inst_split));
    inst.unwrap();
}

#[test]
fn test_unwrap_empty_look() {
    let empty_look = InstEmptyLook {};
    let inst = MaybeInst::Compiled(Inst::EmptyLook(empty_look));
    inst.unwrap();
}

#[test]
fn test_unwrap_char() {
    let inst_char = InstChar { c: 'a' };
    let inst = MaybeInst::Compiled(Inst::Char(inst_char));
    inst.unwrap();
}

#[test]
fn test_unwrap_ranges() {
    let ranges = vec![('a', 'z')];
    let inst_ranges = InstRanges { ranges };
    let inst = MaybeInst::Compiled(Inst::Ranges(inst_ranges));
    inst.unwrap();
}

#[test]
fn test_unwrap_bytes() {
    let inst_bytes = InstBytes { start: 0x00, end: 0xFF };
    let inst = MaybeInst::Compiled(Inst::Bytes(inst_bytes));
    inst.unwrap();
}

