// Answer 0

#[test]
fn test_end_function_success() {
    struct TestStruct {
        void: !,
    }
    
    let result: Result<(), serde::ser::Error> = TestStruct { void: () }.end();
    assert!(result.is_err());
}

#[test]
#[should_panic]
fn test_end_function_panic() {
    struct TestStruct {
        void: !,
    }
    
    let result: Result<(), serde::ser::Error> = TestStruct { void: panic!() }.end();
}

