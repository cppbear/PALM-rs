// Answer 0

#[test]
fn test_end_with_non_empty_vec() {
    struct TestStruct {
        vec: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let input = TestStruct {
        vec: vec![1, 2, 3],
    };

    let result = input.end();
    assert_eq!(result, Ok(Value::Array(vec![Value::Number(1.into()), Value::Number(2.into()), Value::Number(3.into())])));
}

#[test]
fn test_end_with_empty_vec() {
    struct TestStruct {
        vec: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let input = TestStruct {
        vec: vec![],
    };

    let result = input.end();
    assert_eq!(result, Ok(Value::Array(vec![])));
}

#[test]
fn test_end_with_large_vec() {
    struct TestStruct {
        vec: Vec<i32>,
    }

    impl TestStruct {
        fn end(self) -> Result<Value> {
            Ok(Value::Array(self.vec))
        }
    }

    let input = TestStruct {
        vec: (0..1000).collect(),
    };

    let result = input.end();
    let expected: Vec<Value> = (0..1000).map(Value::Number).collect();
    assert_eq!(result, Ok(Value::Array(expected)));
}

