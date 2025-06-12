// Answer 0

#[test]
fn test_c_empty_look_start_line() {
    struct TestHir;
    
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::StartLine);
    
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::One(_)));
    assert_eq!(patch.entry, compiler.insts.len() - 1);
}

#[test]
fn test_c_empty_look_end_line() {
    struct TestHir;
    
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::EndLine);
    
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::One(_)));
    assert_eq!(patch.entry, compiler.insts.len() - 1);
}

#[test]
fn test_c_empty_look_start_text() {
    struct TestHir;
    
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::StartText);
    
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::One(_)));
    assert_eq!(patch.entry, compiler.insts.len() - 1);
}

#[test]
fn test_c_empty_look_word_boundary() {
    struct TestHir;
    
    let mut compiler = Compiler::new();
    let result = compiler.c_empty_look(EmptyLook::WordBoundary);
    
    assert!(result.is_ok());
    let patch = result.unwrap();
    assert!(matches!(patch.hole, Hole::One(_)));
    assert_eq!(patch.entry, compiler.insts.len() - 1);
}

