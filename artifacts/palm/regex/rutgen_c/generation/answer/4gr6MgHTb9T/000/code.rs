// Answer 0

#[test]
fn test_compile_finish_no_instructions() {
    let compiler = Compiler::new();
    let result = compiler.compile_finish();
    assert!(result.is_ok());
    let program = result.unwrap();
    assert!(program.insts.is_empty());
    assert!(program.byte_classes.is_empty());
    assert!(program.capture_name_idx.is_empty());
}

#[test]
fn test_compile_finish_with_instructions() {
    let mut compiler = Compiler::new();
    
    // Adding a dummy instruction for testing.
    compiler.insts.push(MaybeInst::Compiled(Inst::Match(0)));
    
    let result = compiler.compile_finish();
    assert!(result.is_ok());
    let program = result.unwrap();

    assert_eq!(program.insts.len(), 1);
    assert_eq!(matches!(program.insts[0], Inst::Match(0)), true);
    assert!(program.byte_classes.is_empty());
    assert!(program.capture_name_idx.is_empty());
}

#[test]
fn test_compile_finish_with_byte_classes() {
    let mut compiler = Compiler::new();
    compiler.byte_classes.set_range(1, 5); // setting some byte class ranges
    
    let result = compiler.compile_finish();
    assert!(result.is_ok());
    let program = result.unwrap();

    assert!(program.byte_classes.len() > 0); // Ensure some byte classes are populated
    assert!(program.capture_name_idx.is_empty());
}

