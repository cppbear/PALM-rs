// Answer 0

#[test]
fn test_compile_empty_expression() {
    let mut compiler = Compiler::new();
    let expr = Hir::new_empty();
    compiler.c(&expr);
}

#[test]
fn test_compile_literal_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_literal_unicode('a');
    compiler.c(&expr);
}

#[test]
fn test_compile_literal_byte() {
    let mut compiler = Compiler::new().bytes(true);
    let expr = Hir::from_literal_byte(b'a');
    compiler.c(&expr);
}

#[test]
fn test_compile_unicode_class() {
    let mut compiler = Compiler::new();
    let cls = Hir::from_unicode_class(vec![(0x61, 0x7A)]);
    compiler.c(&cls);
}

#[test]
fn test_compile_bytes_class() {
    let mut compiler = Compiler::new().bytes(true);
    let cls = Hir::from_bytes_class(vec![(b'a', b'z')]);
    compiler.c(&cls);
}

#[test]
fn test_compile_anchor_start_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_anchor_start_line();
    compiler.c(&expr);
}

#[test]
fn test_compile_anchor_end_line() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_anchor_end_line();
    compiler.c(&expr);
}

#[test]
fn test_compile_word_boundary_unicode() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_word_boundary_unicode();
    compiler.c(&expr);
}

#[test]
fn test_compile_concatenation() {
    let mut compiler = Compiler::new();
    let expr1 = Hir::from_literal_unicode('a');
    let expr2 = Hir::from_literal_unicode('b');
    let expr = Hir::from_concat(vec![expr1, expr2]);
    compiler.c(&expr);
}

#[test]
fn test_compile_alternation() {
    let mut compiler = Compiler::new();
    let expr1 = Hir::from_literal_unicode('a');
    let expr2 = Hir::from_literal_unicode('b');
    let expr = Hir::from_alternation(vec![expr1, expr2]);
    compiler.c(&expr);
}

#[test]
fn test_compile_repetition_zero_or_more() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_repetition_zero_or_more(Hir::from_literal_unicode('a'));
    compiler.c(&expr);
}

#[test]
fn test_compile_repetition_one_or_more() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_repetition_one_or_more(Hir::from_literal_unicode('b'));
    compiler.c(&expr);
}

#[test]
fn test_compile_repetition_bounded() {
    let mut compiler = Compiler::new();
    let expr = Hir::from_repetition_bounded(Hir::from_literal_unicode('c'), 2, 5);
    compiler.c(&expr);
}

#[test]
fn test_compile_large_size_limit() {
    let mut compiler = Compiler::new().size_limit(20 * (1 << 20));
    let expr = Hir::from_large_expression();
    compiler.c(&expr);
}

#[test]
#[should_panic]
fn test_compile_exceed_size_limit() {
    let mut compiler = Compiler::new().size_limit(1);
    let expr = Hir::from_large_expression_over_limit();
    compiler.c(&expr);
}

