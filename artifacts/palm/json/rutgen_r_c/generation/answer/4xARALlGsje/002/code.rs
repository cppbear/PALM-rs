// Answer 0

#[test]
fn test_index_into_with_valid_index() {
    struct Indexable(usize);

    impl Index for Indexable {
        // Implementation of other trait methods can be omitted as they aren't tested
    }

    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = Indexable(1);
    assert_eq!(index.index_into(&value), Some(&Value::Bool(false)));
}

#[test]
fn test_index_into_with_zero_index() {
    struct Indexable(usize);

    impl Index for Indexable {
        // Implementation of other trait methods can be omitted as they aren't tested
    }

    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = Indexable(0);
    assert_eq!(index.index_into(&value), Some(&Value::Bool(true)));
}

#[test]
fn test_index_into_with_out_of_bounds_index() {
    struct Indexable(usize);

    impl Index for Indexable {
        // Implementation of other trait methods can be omitted as they aren't tested
    }

    let value = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let index = Indexable(2);
    assert_eq!(index.index_into(&value), None);
}

#[test]
fn test_index_into_with_empty_array() {
    struct Indexable(usize);

    impl Index for Indexable {
        // Implementation of other trait methods can be omitted as they aren't tested
    }

    let value = Value::Array(vec![]);
    let index = Indexable(0);
    assert_eq!(index.index_into(&value), None);
}

#[test]
fn test_index_into_with_non_array_value() {
    struct Indexable(usize);

    impl Index for Indexable {
        // Implementation of other trait methods can be omitted as they aren't tested
    }

    let value = Value::Bool(true);
    let index = Indexable(0);
    assert_eq!(index.index_into(&value), None);
}

