// Answer 0

fn test_c_class_with_non_empty_ranges_bytes() {
    let mut compiler = Compiler::new().bytes(true);
    
    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'a'), // simple range for a single character
        hir::ClassUnicodeRange::new('b', 'd'), // multiple character range
    ];

    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
    
    if let Ok(patch) = result {
        assert_eq!(patch.entry, 0); // the entry should be the index of the first instruction
        assert!(matches!(patch.hole, Hole::One(_))); // should return a hole, expect it to be one
    }
}

fn test_c_class_with_single_character_range_bytes() {
    let mut compiler = Compiler::new().bytes(true);
    
    let ranges = vec![
        hir::ClassUnicodeRange::new('x', 'x'), // single character range
    ];

    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
    
    if let Ok(patch) = result {
        assert_eq!(patch.entry, 0); // entry should be the index of the first instruction
        assert!(matches!(patch.hole, Hole::One(_))); // single character should create a hole
    }
}

fn test_c_class_with_multiple_disjoint_ranges_bytes() {
    let mut compiler = Compiler::new().bytes(true);
    
    let ranges = vec![
        hir::ClassUnicodeRange::new('a', 'b'), // range from 'a' to 'b'
        hir::ClassUnicodeRange::new('d', 'f'), // range from 'd' to 'f'
    ];

    let result = compiler.c_class(&ranges);
    assert!(result.is_ok());
    
    if let Ok(patch) = result {
        assert_eq!(patch.entry, 0);
        assert!(matches!(patch.hole, Hole::Many(_))); // should handle multiple holes
    }
}

fn test_c_class_with_empty_ranges_panic() {
    let mut compiler = Compiler::new().bytes(true);
    
    let ranges: Vec<hir::ClassUnicodeRange> = vec![];

    let result = std::panic::catch_unwind(|| {
        compiler.c_class(&ranges).unwrap();
    });

    assert!(result.is_err()); // should panic due to empty ranges
}

