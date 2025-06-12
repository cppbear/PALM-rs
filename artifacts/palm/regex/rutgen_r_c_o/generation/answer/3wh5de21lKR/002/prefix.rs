// Answer 0

#[test]
fn test_c_class_bytes_single_range() {
    let mut compiler = Compiler::new();
    let ranges = vec![hir::ClassBytesRange::new(0, 0)];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_multiple_single_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(1, 1),
        hir::ClassBytesRange::new(2, 2),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_overlapping_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(4, 5),
        hir::ClassBytesRange::new(6, 7),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_non_contiguous_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(8, 9),
        hir::ClassBytesRange::new(10, 15),
        hir::ClassBytesRange::new(16, 31),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_full_range() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(32, 255),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_combined_ranges() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(0, 0),
        hir::ClassBytesRange::new(2, 2),
        hir::ClassBytesRange::new(4, 5),
        hir::ClassBytesRange::new(10, 15),
        hir::ClassBytesRange::new(32, 255),
    ];
    compiler.c_class_bytes(&ranges);
}

