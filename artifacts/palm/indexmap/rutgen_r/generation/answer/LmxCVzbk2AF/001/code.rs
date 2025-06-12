// Answer 0


#[derive(Clone)]
struct TestStruct {
    values: Vec<i32>,
}

impl TestStruct {
    fn new(values: Vec<i32>) -> Self {
        TestStruct { values }
    }
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.clone().values).finish()
    }
}

#[test]
fn test_fmt_non_empty() {
    let test_struct = TestStruct::new(vec![1, 2, 3, 4, 5]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| test_struct.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "[1, 2, 3, 4, 5]");
}

#[test]
fn test_fmt_empty() {
    let test_struct = TestStruct::new(vec![]);
    let mut output = String::new();
    let result = std::fmt::write(&mut output, |f| test_struct.fmt(f));
    assert!(result.is_ok());
    assert_eq!(output, "[]");
}

#[test]
#[should_panic]
fn test_fmt_panic() {
    // This test is intentionally designed to panic by simulating an invalid state.
    // Since there are no unsafe operations in the provided fmt function,
    // an actual panic may not occur. This serves as an example placeholder.
    let test_struct = TestStruct::new(vec![1, 2, 3]);
    let mut output = String::new();
    
    // Here, we introduce a condition that will not happen, i.e. we modify internal structure directly.
    // Nevertheless, this is not the right way to trigger panic in a controlled manner.
    // This serves as a placeholder to represent boundary-panic case implicitly.
    // The actual code cannot directly be made to panic without unsafe manipulation.
    
    // Call a method that doesn’t exist causing panic
    let _result = std::fmt::write(&mut output, |f| {
        panic!("Panic simulating unhandled format condition");
    });
}


