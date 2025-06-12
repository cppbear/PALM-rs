// Answer 0

#[test]
#[should_panic]
fn test_c_class_empty_ranges() {
    let mut compiler = Compiler::new();
    let ranges: Vec<hir::ClassUnicodeRange> = vec![];
    compiler.c_class(&ranges);
}

