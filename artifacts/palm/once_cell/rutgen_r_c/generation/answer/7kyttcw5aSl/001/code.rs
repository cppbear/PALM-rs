// Answer 0

#[test]
fn test_init_function_err_case() {
    struct Test;
    
    impl Test {
        fn trigger_error(&self) -> Result<&'static str, &'static str> {
            Err("Error occurred")
        }
    }

    let once_ref = OnceRef::<Test>::new();
    let result: Result<&Test, &'static str> = once_ref.init(|_| Test.trigger_error(&Test));
    assert_eq!(result, Err("Error occurred"));
}

#[test]
fn test_init_function_success_case() {
    struct Test;
    
    impl Test {
        fn get_instance() -> &'static str {
            "Instance"
        }
    }

    let once_ref = OnceRef::<Test>::new();
    let result: Result<&Test, ()> = once_ref.init(|_| Ok(Test.get_instance()));
    assert_eq!(result.unwrap(), Test.get_instance());
}

#[test]
fn test_init_function_double_init() {
    struct Test {
        value: &'static str,
    }
    
    impl Test {
        fn new() -> &'static str {
            "Initialized"
        }
    }

    let once_ref = OnceRef::<Test>::new();
    let _ = once_ref.init(|_| Ok(Test::new()));
    
    // This should not trigger an error, the result should be the already initialized value
    let result = once_ref.init(|_| Ok(Test::new()));
    assert_eq!(result.unwrap(), Test::new());
}

#[test]
#[should_panic]
fn test_init_function_panics_on_non_static() {
    struct NonStatic;

    impl NonStatic {
        fn create() -> Result<&'static str, &'static str> {
            let value = String::from("Non-static");
            Ok(Box::leak(value.into_boxed_str()))
        }
    }

    let once_ref = OnceRef::<NonStatic>::new();
    let _ = once_ref.init(|_| NonStatic::create());
}

