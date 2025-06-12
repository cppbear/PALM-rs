// Answer 0

#[test]
fn test_is_all_ascii_empty() {
    struct CharClass {
        set: Set,
    }

    struct Set {
        intervals: Vec<Range>,
    }

    struct Range {
        end: u32,
    }

    impl CharClass {
        fn is_all_ascii(&self) -> bool {
            self.set.intervals.last().map_or(true, |r| r.end <= 0x7F)
        }
    }

    let char_class = CharClass { set: Set { intervals: Vec::new() } };
    
    assert!(char_class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_only_ascii() {
    struct CharClass {
        set: Set,
    }

    struct Set {
        intervals: Vec<Range>,
    }

    struct Range {
        end: u32,
    }

    impl CharClass {
        fn is_all_ascii(&self) -> bool {
            self.set.intervals.last().map_or(true, |r| r.end <= 0x7F)
        }
    }

    let char_class = CharClass { set: Set { intervals: vec![Range { end: 0x7F }] } };
    
    assert!(char_class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_contains_non_ascii() {
    struct CharClass {
        set: Set,
    }

    struct Set {
        intervals: Vec<Range>,
    }

    struct Range {
        end: u32,
    }

    impl CharClass {
        fn is_all_ascii(&self) -> bool {
            self.set.intervals.last().map_or(true, |r| r.end <= 0x7F)
        }
    }

    let char_class = CharClass { set: Set { intervals: vec![Range { end: 0x80 }] } };
    
    assert!(!char_class.is_all_ascii());
} 

#[test]
fn test_is_all_ascii_multiple_intervals() {
    struct CharClass {
        set: Set,
    }

    struct Set {
        intervals: Vec<Range>,
    }

    struct Range {
        end: u32,
    }

    impl CharClass {
        fn is_all_ascii(&self) -> bool {
            self.set.intervals.last().map_or(true, |r| r.end <= 0x7F)
        }
    }

    let char_class = CharClass { set: Set { intervals: vec![Range { end: 0x50 }, Range { end: 0x7A }] } };
    
    assert!(char_class.is_all_ascii());
}

#[test]
fn test_is_all_ascii_multilevel_non_ascii() {
    struct CharClass {
        set: Set,
    }

    struct Set {
        intervals: Vec<Range>,
    }

    struct Range {
        end: u32,
    }

    impl CharClass {
        fn is_all_ascii(&self) -> bool {
            self.set.intervals.last().map_or(true, |r| r.end <= 0x7F)
        }
    }

    let char_class = CharClass { set: Set { intervals: vec![Range { end: 0x70 }, Range { end: 0x80 }] } };
    
    assert!(!char_class.is_all_ascii());
}

