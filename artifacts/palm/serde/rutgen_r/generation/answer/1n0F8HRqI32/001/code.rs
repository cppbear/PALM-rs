// Answer 0

#[test]
fn test_visit_borrowed_str_success() {
    struct MyError;
    impl serde::de::Error for MyError {
        fn custom<T>(_msg: T) -> Self {
            MyError
        }
    }

    struct MyVisitor;

    impl MyVisitor {
        fn visit_borrowed_str<'a, E>(self, v: &'a str) -> Result<&'a str, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = MyVisitor;
    let input_str = "test string";
    let result: Result<&str, MyError> = visitor.visit_borrowed_str(input_str);
    assert_eq!(result, Ok(input_str));
}

#[test]
fn test_visit_borrowed_str_empty_string() {
    struct MyError;
    impl serde::de::Error for MyError {
        fn custom<T>(_msg: T) -> Self {
            MyError
        }
    }

    struct MyVisitor;

    impl MyVisitor {
        fn visit_borrowed_str<'a, E>(self, v: &'a str) -> Result<&'a str, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }
    }

    let visitor = MyVisitor;
    let input_str = "";
    let result: Result<&str, MyError> = visitor.visit_borrowed_str(input_str);
    assert_eq!(result, Ok(input_str));
}

