// Answer 0

#[test]
fn test_as_display() {
    struct Displayable(i32);
    
    impl Display for Displayable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    let value = Displayable(42);
    let reference: &Displayable = &value;
    
    assert_eq!(reference.as_display().to_string(), "42");
}

#[test]
fn test_as_display_with_dyn() {
    struct Displayable(i32);
    
    impl Display for Displayable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    let value: &dyn Display = &Displayable(100);
    
    assert_eq!(value.to_string(), "100");
}

