// Answer 0

#[test]
fn test_c_class_bytes_case1() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(1, 2),
        hir::ClassBytesRange::new(3, 4),
        hir::ClassBytesRange::new(5, 6),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_case2() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(0, 1),
        hir::ClassBytesRange::new(2, 255),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_case3() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(10, 20),
        hir::ClassBytesRange::new(30, 40),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_case4() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(100, 200),
        hir::ClassBytesRange::new(250, 255),
    ];
    compiler.c_class_bytes(&ranges);
}

#[test]
fn test_c_class_bytes_case5() {
    let mut compiler = Compiler::new();
    let ranges = vec![
        hir::ClassBytesRange::new(50, 100),
        hir::ClassBytesRange::new(200, 250),
    ];
    compiler.c_class_bytes(&ranges);
}

