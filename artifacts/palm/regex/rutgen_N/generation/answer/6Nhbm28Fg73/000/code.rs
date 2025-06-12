// Answer 0

#[derive(Debug)]
struct ClassBytesSet {
    set: Vec<ClassBytesRange>,
}

#[derive(Debug)]
struct ClassBytesRange {
    start: u8,
    end: u8,
}

impl ClassBytesSet {
    pub fn new() -> Self {
        ClassBytesSet { set: Vec::new() }
    }

    pub fn push(&mut self, range: ClassBytesRange) {
        self.set.push(range);
    }
}

#[test]
fn test_push_single_range() {
    let mut class_bytes_set = ClassBytesSet::new();
    let range = ClassBytesRange { start: 1, end: 5 };
    
    class_bytes_set.push(range);
    
    assert_eq!(class_bytes_set.set.len(), 1);
    assert_eq!(class_bytes_set.set[0].start, 1);
    assert_eq!(class_bytes_set.set[0].end, 5);
}

#[test]
fn test_push_multiple_ranges() {
    let mut class_bytes_set = ClassBytesSet::new();
    
    class_bytes_set.push(ClassBytesRange { start: 1, end: 5 });
    class_bytes_set.push(ClassBytesRange { start: 6, end: 10 });
    
    assert_eq!(class_bytes_set.set.len(), 2);
    assert_eq!(class_bytes_set.set[0].start, 1);
    assert_eq!(class_bytes_set.set[0].end, 5);
    assert_eq!(class_bytes_set.set[1].start, 6);
    assert_eq!(class_bytes_set.set[1].end, 10);
}

#[test]
fn test_push_boundary_ranges() {
    let mut class_bytes_set = ClassBytesSet::new();
    let range1 = ClassBytesRange { start: 0, end: 0 };
    let range2 = ClassBytesRange { start: 255, end: 255 };
    
    class_bytes_set.push(range1);
    class_bytes_set.push(range2);
    
    assert_eq!(class_bytes_set.set.len(), 2);
    assert_eq!(class_bytes_set.set[0].start, 0);
    assert_eq!(class_bytes_set.set[0].end, 0);
    assert_eq!(class_bytes_set.set[1].start, 255);
    assert_eq!(class_bytes_set.set[1].end, 255);
}

