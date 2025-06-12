// Answer 0

#[test]
fn test_visit_str_normal() {
    struct TestVisitor<'a>(&'a mut String);
    
    impl<'a, 'de> Visitor<'de> for TestVisitor<'a> {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
        
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.0.clear();
            self.0.push_str(v);
            Ok(())
        }
    }

    let mut output = String::new();
    let visitor = TestVisitor(&mut output);
    
    let result: Result<(), ()> = visitor.visit_str("Hello, world!");
    assert!(result.is_ok());
    assert_eq!(output, "Hello, world!");
}

#[test]
fn test_visit_str_with_clear() {
    struct TestVisitor<'a>(&'a mut String);
    
    impl<'a, 'de> Visitor<'de> for TestVisitor<'a> {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
        
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.0.clear();
            self.0.push_str(v);
            Ok(())
        }
    }

    let mut output = String::from("Initial value");
    let visitor = TestVisitor(&mut output);
    
    let result: Result<(), ()> = visitor.visit_str("New value");
    assert!(result.is_ok());
    assert_eq!(output, "New value");
}

#[test]
#[should_panic]
fn test_visit_str_invalid_utf8() {
    struct TestVisitor<'a>(&'a mut String);
    
    impl<'a, 'de> Visitor<'de> for TestVisitor<'a> {
        type Value = ();
        
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
        
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => {
                    self.0.clear();
                    self.0.push_str(s);
                    Ok(())
                }
                Err(_) => {
                    panic!("Invalid UTF-8 sequence");
                }
            }
        }
    }

    let mut output = String::new();
    let visitor = TestVisitor(&mut output);

    visitor.visit_bytes(&[0xFF]); // Invalid UTF-8
}

