// Answer 0

#[test]
fn test_fmt_object_panic() {
    struct MockFormatter {
        should_panic: bool,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            if self.should_panic {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    let mut formatter = MockFormatter { should_panic: true };

    let value = Value::Object(Map { 
        map: std::collections::BTreeMap::new() // using BTreeMap as placeholder for MapImpl
    });

    let result = value.fmt(&mut formatter);
    assert!(result.is_err());
}

#[test]
fn test_fmt_object_no_panic() {
    struct MockFormatter {
        outputs: Vec<String>,
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, val: &str) -> fmt::Result {
            self.outputs.push(val.to_string());
            Ok(())
        }
    }

    let mut formatter = MockFormatter { outputs: vec![] };

    let value = Value::Object(Map { 
        map: std::collections::BTreeMap::new() // using BTreeMap as placeholder for MapImpl
    });

    let result = value.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.outputs.len(), 2);
    assert_eq!(formatter.outputs[0], "Object "); // expected first part
}

