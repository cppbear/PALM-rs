// Answer 0

#[test]
fn test_serialize_arguments() {
    use std::fmt::{self, Arguments};
    use std::marker::PhantomData;

    struct TestSerializer {
        result: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                result: String::new(),
            }
        }

        fn collect_str(&mut self, args: &fmt::Arguments) -> Result<(), String> {
            self.result.push_str(&args.to_string());
            Ok(())
        }
    }

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = String;
        
        fn collect_str(self, args: &fmt::Arguments) -> Result<Self::Ok, Self::Error> {
            self.collect_str(args)
        }
    }

    let mut serializer = TestSerializer::new();
    let value = format!("Hello, {}!", "world");
    let args = format_args!("{}", value);

    let result = args.serialize(&mut serializer);
    assert!(result.is_ok());
    assert_eq!(serializer.result, "Hello, world!");
}

#[test]
fn test_serialize_arguments_empty() {
    use std::fmt::{self, Arguments};
    use std::marker::PhantomData;

    struct TestSerializer {
        result: String,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                result: String::new(),
            }
        }

        fn collect_str(&mut self, args: &fmt::Arguments) -> Result<(), String> {
            self.result.push_str(&args.to_string());
            Ok(())
        }
    }

    impl serde::ser::Serializer for TestSerializer {
        type Ok = ();
        type Error = String;

        fn collect_str(self, args: &fmt::Arguments) -> Result<Self::Ok, Self::Error> {
            self.collect_str(args)
        }
    }

    let mut serializer = TestSerializer::new();
    let args = format_args!("");

    let result = args.serialize(&mut serializer);
    assert!(result.is_ok());
    assert_eq!(serializer.result, "");
}

