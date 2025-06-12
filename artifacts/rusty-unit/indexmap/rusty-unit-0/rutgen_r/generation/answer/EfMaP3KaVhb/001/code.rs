// Answer 0

#[test]
fn test_fmt_with_empty_data() {
    struct DebugStruct(Vec<i32>);
    
    impl std::fmt::Debug for DebugStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    
    let data = DebugStruct(vec![]);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = data.fmt(&mut formatter);
        assert!(result.is_ok());
        output = format!("{}", formatter);
    }
    assert_eq!(output, "[]");
}

#[test]
fn test_fmt_with_single_element() {
    struct DebugStruct(Vec<i32>);
    
    impl std::fmt::Debug for DebugStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    
    let data = DebugStruct(vec![42]);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = data.fmt(&mut formatter);
        assert!(result.is_ok());
        output = format!("{}", formatter);
    }
    assert_eq!(output, "[42]");
}

#[test]
fn test_fmt_with_multiple_elements() {
    struct DebugStruct(Vec<i32>);
    
    impl std::fmt::Debug for DebugStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    
    let data = DebugStruct(vec![1, 2, 3, 4, 5]);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = data.fmt(&mut formatter);
        assert!(result.is_ok());
        output = format!("{}", formatter);
    }
    assert_eq!(output, "[1, 2, 3, 4, 5]");
}

#[test]
fn test_fmt_with_negative_elements() {
    struct DebugStruct(Vec<i32>);
    
    impl std::fmt::Debug for DebugStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    
    let data = DebugStruct(vec![-1, -2, -3]);
    let mut output = String::new();
    {
        let mut formatter = std::fmt::Formatter::new();
        let result = data.fmt(&mut formatter);
        assert!(result.is_ok());
        output = format!("{}", formatter);
    }
    assert_eq!(output, "[-1, -2, -3]");
}

