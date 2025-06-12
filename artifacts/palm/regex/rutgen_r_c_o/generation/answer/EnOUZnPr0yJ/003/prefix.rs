// Answer 0

#[test]
fn test_c_class_single_character_range() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a'), hir::ClassUnicodeRange::new('b', 'c')];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_multiple_ranges_with_single_character() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('x', 'y'), hir::ClassUnicodeRange::new('y', 'y'), hir::ClassUnicodeRange::new('z', 'z')];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_all_single_character_ranges() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a'), hir::ClassUnicodeRange::new('b', 'b'), hir::ClassUnicodeRange::new('c', 'c')];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_adjacent_ranges_with_single_characters() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('a', 'a'), hir::ClassUnicodeRange::new('b', 'd')];
    let _result = compiler.c_class(&ranges);
}

#[test]
fn test_c_class_combined_single_and_multiple_characters() {
    let mut compiler = Compiler::new();
    compiler.compiled.is_bytes = false;

    let ranges = vec![hir::ClassUnicodeRange::new('e', 'e'), hir::ClassUnicodeRange::new('f', 'g')];
    let _result = compiler.c_class(&ranges);
}

