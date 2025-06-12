// Answer 0

#[test]
fn test_cls_byte_count_empty() {
    struct ClassBytes {
        bytes: Vec<Range<u8>>,
    }
    
    impl ClassBytes {
        fn iter(&self) -> std::slice::Iter<Range<u8>> {
            self.bytes.iter()
        }
    }
    
    let cls = ClassBytes { bytes: vec![] };
    assert_eq!(cls_byte_count(&cls), 0);
}

#[test]
fn test_cls_byte_count_single_range() {
    struct ClassBytes {
        bytes: Vec<Range<u8>>,
    }
    
    impl ClassBytes {
        fn iter(&self) -> std::slice::Iter<Range<u8>> {
            self.bytes.iter()
        }
    }
    
    let cls = ClassBytes { bytes: vec![0..5] };
    assert_eq!(cls_byte_count(&cls), 5);
}

#[test]
fn test_cls_byte_count_multiple_ranges() {
    struct ClassBytes {
        bytes: Vec<Range<u8>>,
    }
    
    impl ClassBytes {
        fn iter(&self) -> std::slice::Iter<Range<u8>> {
            self.bytes.iter()
        }
    }
    
    let cls = ClassBytes { bytes: vec![0..3, 5..8] };
    assert_eq!(cls_byte_count(&cls), 8);
}

#[test]
fn test_cls_byte_count_adjacent_ranges() {
    struct ClassBytes {
        bytes: Vec<Range<u8>>,
    }
    
    impl ClassBytes {
        fn iter(&self) -> std::slice::Iter<Range<u8>> {
            self.bytes.iter()
        }
    }
    
    let cls = ClassBytes { bytes: vec![0..2, 2..5] };
    assert_eq!(cls_byte_count(&cls), 5);
}

