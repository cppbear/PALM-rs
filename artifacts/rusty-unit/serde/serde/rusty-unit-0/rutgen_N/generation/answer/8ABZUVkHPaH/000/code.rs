// Answer 0

#[test]
fn test_visit_str_success() {
    struct TestVisitor(String);
    
    impl TestVisitor {
        fn new() -> Self {
            TestVisitor(String::new())
        }
        
        fn visit_str<E>(&mut self, v: &str) -> Result<(), E>
        where
            E: std::fmt::Debug + Default,
        {
            self.0.clear();
            self.0.push_str(v);
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let result: Result<(), _> = visitor.visit_str("hello");
    assert!(result.is_ok());
    assert_eq!(visitor.0, "hello");
}

#[test]
#[should_panic]
fn test_visit_str_empty() {
    struct TestVisitor(String);
    
    impl TestVisitor {
        fn new() -> Self {
            TestVisitor(String::new())
        }
        
        fn visit_str<E>(&mut self, v: &str) -> Result<(), E>
        where
            E: std::fmt::Debug + Default,
        {
            self.0.clear();
            self.0.push_str(v);
            if self.0.is_empty() {
                panic!("String is empty.");
            }
            Ok(())
        }
    }

    let mut visitor = TestVisitor::new();
    let _ = visitor.visit_str("");
}

