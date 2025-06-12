// Answer 0

#[test]
fn test_compile_empty_repetition() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition(Hir::new_empty(), true); // Represents `(?:)*`
    
    match compiler.c(&expr) {
        Ok(Patch { hole, entry }) => {
            assert_eq!(hole, Hole::None);
            assert_eq!(entry, 0); // Entry point should be the start of insts
        }
        Err(e) => panic!("Expected Ok but got Err: {:?}", e),
    }
}

#[test]
fn test_compile_repetition_one_or_more() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition(Hir::new_literal('a'), false); // Represents `a+`
    
    match compiler.c(&expr) {
        Ok(Patch { hole, entry }) => {
            assert_eq!(hole, Hole::None);
            assert_eq!(entry, 0); // Entry point should be the start of insts
        }
        Err(e) => panic!("Expected Ok but got Err: {:?}", e),
    }
}

#[test]
fn test_compile_repetition_zero_or_more() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition(Hir::new_literal('b'), true); // Represents `b*`
    
    match compiler.c(&expr) {
        Ok(Patch { hole, entry }) => {
            assert_eq!(hole, Hole::None);
            assert_eq!(entry, 0); // Entry point should be the start of insts
        }
        Err(e) => panic!("Expected Ok but got Err: {:?}", e),
    }
}

#[test]
fn test_compile_repetition_range() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition_range(Hir::new_literal('c'), true, 2, 4); // Represents `c{2,4}`
    
    match compiler.c(&expr) {
        Ok(Patch { hole, entry }) => {
            assert_eq!(hole, Hole::None);
            assert_eq!(entry, 0); // Entry point should be the start of insts
        }
        Err(e) => panic!("Expected Ok but got Err: {:?}", e),
    }
}

#[test]
fn test_compile_repetition_at_least() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_repetition_at_least(Hir::new_literal('d'), true, 1); // Represents `d+`
    
    match compiler.c(&expr) {
        Ok(Patch { hole, entry }) => {
            assert_eq!(hole, Hole::None);
            assert_eq!(entry, 0); // Entry point should be the start of insts
        }
        Err(e) => panic!("Expected Ok but got Err: {:?}", e),
    }
}

