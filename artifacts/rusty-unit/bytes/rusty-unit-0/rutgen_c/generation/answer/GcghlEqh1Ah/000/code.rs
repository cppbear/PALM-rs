// Answer 0

#[test]
fn test_into_inner() {
    struct InnerStruct {
        value: usize,
    }

    let inner = InnerStruct { value: 42 };
    let limit = Limit { inner, limit: 100 };

    let inner_value = limit.into_inner();
    assert_eq!(inner_value.value, 42);
}

#[test]
fn test_into_inner_empty() {
    struct InnerStruct {
        value: usize,
    }

    let inner = InnerStruct { value: 0 };
    let limit = Limit { inner, limit: 100 };

    let inner_value = limit.into_inner();
    assert_eq!(inner_value.value, 0);
}

