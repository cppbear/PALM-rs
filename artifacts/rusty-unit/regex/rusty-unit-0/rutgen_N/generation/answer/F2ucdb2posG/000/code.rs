// Answer 0

#[test]
fn test_hir_class_single_range() {
    struct DummyHir;
    
    impl DummyHir {
        fn new(ranges: Vec<hir::ClassUnicodeRange>) -> hir::ClassUnicode {
            // Assume this constructs a proper ClassUnicode for our test.
            hir::ClassUnicode { ranges }
        }
    }

    let ranges = vec![('a', 'z')];
    let result = hir_class(&ranges);
    assert_eq!(result.ranges.len(), 1);
    assert_eq!(result.ranges[0].start, 'a');
    assert_eq!(result.ranges[0].end, 'z');
}

#[test]
fn test_hir_class_multiple_ranges() {
    struct DummyHir;

    impl DummyHir {
        fn new(ranges: Vec<hir::ClassUnicodeRange>) -> hir::ClassUnicode {
            // Assume this constructs a proper ClassUnicode for our test.
            hir::ClassUnicode { ranges }
        }
    }

    let ranges = vec![('0', '9'), ('A', 'Z'), ('a', 'z')];
    let result = hir_class(&ranges);
    assert_eq!(result.ranges.len(), 3);
    assert_eq!(result.ranges[0].start, '0');
    assert_eq!(result.ranges[0].end, '9');
    assert_eq!(result.ranges[1].start, 'A');
    assert_eq!(result.ranges[1].end, 'Z');
    assert_eq!(result.ranges[2].start, 'a');
    assert_eq!(result.ranges[2].end, 'z');
}

#[test]
fn test_hir_class_empty_ranges() {
    struct DummyHir;

    impl DummyHir {
        fn new(ranges: Vec<hir::ClassUnicodeRange>) -> hir::ClassUnicode {
            // Assume this constructs a proper ClassUnicode for our test.
            hir::ClassUnicode { ranges }
        }
    }

    let ranges: Vec<(char, char)> = Vec::new();
    let result = hir_class(&ranges);
    assert_eq!(result.ranges.len(), 0);
}

#[test]
#[should_panic]
fn test_hir_class_invalid_range() {
    struct DummyHir;

    impl DummyHir {
        fn new(ranges: Vec<hir::ClassUnicodeRange>) -> hir::ClassUnicode {
            // Assume this constructs a proper ClassUnicode for our test.
            hir::ClassUnicode { ranges }
        }
    }

    let ranges = vec![('z', 'a')]; // Invalid range
    let _result = hir_class(&ranges); // This should panic
}

