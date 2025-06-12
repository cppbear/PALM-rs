// Answer 0

#[test]
fn test_fmt() {
    struct TestAst;

    impl std::fmt::Display for TestAst {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            // Simulate the output of the Printer's print method.
            write!(f, "TestAst")
        }
    }

    let ast = TestAst;
    let mut output = String::new();
    let result = ast.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "TestAst");
}

