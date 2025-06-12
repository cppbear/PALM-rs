// Answer 0

#[test]
fn test_unwrap_compiled_inst() {
    let compiled_inst = Inst::Match(0);
    let maybe_inst = MaybeInst::Compiled(compiled_inst.clone());
    assert_eq!(maybe_inst.unwrap(), compiled_inst);
}

#[should_panic(expected = "must be called on a compiled instruction")]
#[test]
fn test_unwrap_non_compiled_inst() {
    let non_compiled_inst = MaybeInst::Split;
    non_compiled_inst.unwrap();
}

#[test]
fn test_unwrap_with_different_inst_types() {
    let compiled_inst_save = MaybeInst::Compiled(Inst::Save(InstSave { slot: 1 }));
    assert_eq!(compiled_inst_save.unwrap(), Inst::Save(InstSave { slot: 1 }));

    let compiled_inst_char = MaybeInst::Compiled(Inst::Char(InstChar { c: 'a' }));
    assert_eq!(compiled_inst_char.unwrap(), Inst::Char(InstChar { c: 'a' }));
}

