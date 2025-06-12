// Answer 0

#[test]
fn test_complete_true_non_empty() {
    struct Literal {
        complete: bool,
        empty: bool,
    }
    
    impl Literal {
        pub fn complete(&self) -> bool {
            self.complete && !self.is_empty()
        }

        pub fn is_empty(&self) -> bool {
            self.empty
        }
    }
    
    let lit = Literal { complete: true, empty: false };
    assert!(lit.complete());
}

#[test]
fn test_complete_true_empty() {
    struct Literal {
        complete: bool,
        empty: bool,
    }
    
    impl Literal {
        pub fn complete(&self) -> bool {
            self.complete && !self.is_empty()
        }

        pub fn is_empty(&self) -> bool {
            self.empty
        }
    }

    let lit = Literal { complete: true, empty: true };
    assert!(!lit.complete());
}

#[test]
fn test_complete_false_non_empty() {
    struct Literal {
        complete: bool,
        empty: bool,
    }
    
    impl Literal {
        pub fn complete(&self) -> bool {
            self.complete && !self.is_empty()
        }

        pub fn is_empty(&self) -> bool {
            self.empty
        }
    }

    let lit = Literal { complete: false, empty: false };
    assert!(!lit.complete());
}

#[test]
fn test_complete_false_empty() {
    struct Literal {
        complete: bool,
        empty: bool,
    }
    
    impl Literal {
        pub fn complete(&self) -> bool {
            self.complete && !self.is_empty()
        }

        pub fn is_empty(&self) -> bool {
            self.empty
        }
    }

    let lit = Literal { complete: false, empty: true };
    assert!(!lit.complete());
}

