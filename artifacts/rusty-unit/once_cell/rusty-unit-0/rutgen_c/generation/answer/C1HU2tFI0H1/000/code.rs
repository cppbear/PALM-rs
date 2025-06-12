// Answer 0

#[cfg(test)]
fn test_get_or_try_init() {
    struct TestOnceBool {
        once_bool: OnceBool,
    }
    
    impl TestOnceBool {
        fn new() -> Self {
            Self {
                once_bool: OnceBool::new(),
            }
        }
    }

    let test_instance = TestOnceBool::new();

    let result = test_instance.once_bool.get_or_try_init(|| Ok(true));
    assert_eq!(result.unwrap(), true);

    let result_again = test_instance.once_bool.get_or_try_init(|| Ok(false));
    assert_eq!(result_again.unwrap(), true);
    
    let result_error = test_instance.once_bool.get_or_try_init(|| Err("error"));
    assert_eq!(result_error.is_err(), false);
    
    let result_error_second_call = test_instance.once_bool.get_or_try_init(|| {
        panic!("Should not call this function again");
    });
    assert_eq!(result_error_second_call.unwrap(), true);
}

#[cfg(test)]
fn test_get_or_try_init_failure() {
    struct TestOnceBool {
        once_bool: OnceBool,
    }

    impl TestOnceBool {
        fn new() -> Self {
            Self {
                once_bool: OnceBool::new(),
            }
        }
    }

    let test_instance = TestOnceBool::new();

    let result = test_instance.once_bool.get_or_try_init(|| Err("initial error"));
    assert!(result.is_err());

    let result_again = test_instance.once_bool.get_or_try_init(|| Ok(true));
    assert_eq!(result_again.unwrap(), true);
}

