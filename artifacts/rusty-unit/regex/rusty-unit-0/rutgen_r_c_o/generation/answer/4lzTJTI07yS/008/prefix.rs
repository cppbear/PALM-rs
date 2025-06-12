// Answer 0

#[test]
fn test_compile_capture_index_with_no_captures() {
    let mut compiler = Compiler::new().size_limit(100);
    let expr = Hir::new_group(hir::GroupKind::CaptureIndex(0), Hir::new_empty());
    let result = compiler.c(&expr);
}

#[test]
fn test_compile_capture_index_with_one_capture() {
    let mut compiler = Compiler::new().size_limit(100);
    compiler.compiled.captures.push(Some("first".to_string()));
    let expr = Hir::new_group(hir::GroupKind::CaptureIndex(1), Hir::new_empty());
    let result = compiler.c(&expr);
}

#[test]
fn test_compile_capture_index_with_two_captures() {
    let mut compiler = Compiler::new().size_limit(100);
    compiler.compiled.captures.push(Some("first".to_string()));
    compiler.compiled.captures.push(Some("second".to_string()));
    let expr = Hir::new_group(hir::GroupKind::CaptureIndex(2), Hir::new_empty());
    let result = compiler.c(&expr);
}

#[test]
fn test_compile_capture_index_with_three_captures() {
    let mut compiler = Compiler::new().size_limit(100);
    compiler.compiled.captures.push(Some("first".to_string()));
    compiler.compiled.captures.push(Some("second".to_string()));
    compiler.compiled.captures.push(Some("third".to_string()));
    let expr = Hir::new_group(hir::GroupKind::CaptureIndex(3), Hir::new_empty());
    let result = compiler.c(&expr);
}

