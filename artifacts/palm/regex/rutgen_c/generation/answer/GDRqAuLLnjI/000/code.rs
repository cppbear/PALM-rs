// Answer 0

#[test]
fn test_negate_unicode() {
    struct MockClassUnicode {
        negated: bool,
        set: IntervalSet<ClassUnicodeRange>,
    }

    impl MockClassUnicode {
        fn new() -> Self {
            Self { negated: false, set: IntervalSet::new() }
        }
        
        fn negate(&mut self) {
            self.negated = !self.negated;
        }
    }

    let mut class = Class::Unicode(MockClassUnicode::new());
    class.negate();
    if let Class::Unicode(ref x) = class {
        assert!(x.negated);
    }
}

#[test]
fn test_negate_bytes() {
    struct MockClassBytes {
        negated: bool,
        set: IntervalSet<ClassBytesRange>,
    }

    impl MockClassBytes {
        fn new() -> Self {
            Self { negated: false, set: IntervalSet::new() }
        }
        
        fn negate(&mut self) {
            self.negated = !self.negated;
        }
    }

    let mut class = Class::Bytes(MockClassBytes::new());
    class.negate();
    if let Class::Bytes(ref x) = class {
        assert!(x.negated);
    }
}

#[test]
fn test_negate_unicode_twice() {
    struct MockClassUnicode {
        negated: bool,
        set: IntervalSet<ClassUnicodeRange>,
    }

    impl MockClassUnicode {
        fn new() -> Self {
            Self { negated: false, set: IntervalSet::new() }
        }
        
        fn negate(&mut self) {
            self.negated = !self.negated;
        }
    }

    let mut class = Class::Unicode(MockClassUnicode::new());
    class.negate();
    class.negate();
    if let Class::Unicode(ref x) = class {
        assert!(!x.negated);
    }
}

#[test]
fn test_negate_bytes_twice() {
    struct MockClassBytes {
        negated: bool,
        set: IntervalSet<ClassBytesRange>,
    }

    impl MockClassBytes {
        fn new() -> Self {
            Self { negated: false, set: IntervalSet::new() }
        }
        
        fn negate(&mut self) {
            self.negated = !self.negated;
        }
    }

    let mut class = Class::Bytes(MockClassBytes::new());
    class.negate();
    class.negate();
    if let Class::Bytes(ref x) = class {
        assert!(!x.negated);
    }
}

