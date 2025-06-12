// Answer 0

#[test]
fn test_push_compiled_single_instruction() {
    let mut compiler = Compiler::new();
    let instruction = Inst::Match(1);
    compiler.push_compiled(instruction);
    assert_eq!(compiler.insts.len(), 1);
}

#[test]
fn test_push_compiled_multiple_instructions() {
    let mut compiler = Compiler::new();
    let instruction1 = Inst::Match(1);
    let instruction2 = Inst::Char(InstChar { c: 'a' });
    compiler.push_compiled(instruction1);
    compiler.push_compiled(instruction2);
    assert_eq!(compiler.insts.len(), 2);
}

#[test]
fn test_push_compiled_empty_instructions() {
    let mut compiler = Compiler::new();
    assert_eq!(compiler.insts.len(), 0);
    compiler.push_compiled(Inst::Save(InstSave { index: 0 }));
    assert_eq!(compiler.insts.len(), 1);
}

