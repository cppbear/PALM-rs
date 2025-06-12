// Answer 0

#[test]
fn test_clear_empty_set() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    impl LiteralSet {
        fn new() -> Self {
            LiteralSet { lits: Vec::new() }
        }

        pub fn clear(&mut self) {
            self.lits.clear();
        }
    }

    let mut set = LiteralSet::new();
    set.clear();
    assert!(set.lits.is_empty());
}

#[test]
fn test_clear_non_empty_set() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    impl LiteralSet {
        fn new() -> Self {
            LiteralSet { lits: Vec::new() }
        }

        pub fn clear(&mut self) {
            self.lits.clear();
        }
    }

    let mut set = LiteralSet::new();
    set.lits.push("test".to_string());
    set.clear();
    assert!(set.lits.is_empty());
}

#[test]
fn test_clear_after_multiple_additions() {
    struct LiteralSet {
        lits: Vec<String>,
    }

    impl LiteralSet {
        fn new() -> Self {
            LiteralSet { lits: Vec::new() }
        }

        pub fn clear(&mut self) {
            self.lits.clear();
        }
    }

    let mut set = LiteralSet::new();
    set.lits.push("first".to_string());
    set.lits.push("second".to_string());
    set.clear();
    assert!(set.lits.is_empty());
}

