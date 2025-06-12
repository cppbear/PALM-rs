// Answer 0

#[test]
fn partition_point_empty() {
    struct TestKey;
    struct TestValue;

    let slice: Slice<TestKey, TestValue> = Slice { entries: [] };
    let predicate = |_: &TestKey, _: &TestValue| false;

    let result = slice.partition_point(predicate);
    assert_eq!(result, 0);
}

#[test]
fn partition_point_single_element() {
    struct TestKey;
    struct TestValue;

    let entries = [Bucket { hash: 0, key: TestKey, value: TestValue }];
    let slice: Slice<TestKey, TestValue> = Slice { entries };
    let predicate = |_: &TestKey, _: &TestValue| false;

    let result = slice.partition_point(predicate);
    assert_eq!(result, 1);
}

#[test]
fn partition_point_all_true() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];
    let slice: Slice<TestKey, TestValue> = Slice { entries };
    let predicate = |_: &TestKey, _: &TestValue| true;

    let result = slice.partition_point(predicate);
    assert_eq!(result, 3);
}

#[test]
fn partition_point_some_true() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];
    let slice: Slice<TestKey, TestValue> = Slice { entries };

    let predicate = |_: &TestKey, _: &TestValue| true;
    let result = slice.partition_point(predicate);
    assert_eq!(result, 3);
}

#[test]
fn partition_point_boundary_conditions() {
    struct TestKey;
    struct TestValue;

    let entries = [
        Bucket { hash: 0, key: TestKey, value: TestValue },
        Bucket { hash: 1, key: TestKey, value: TestValue },
        Bucket { hash: 2, key: TestKey, value: TestValue },
    ];
    let slice: Slice<TestKey, TestValue> = Slice { entries };

    let predicate_false_on_first = |_: &TestKey, _: &TestValue| false;
    let result = slice.partition_point(predicate_false_on_first);
    assert_eq!(result, 0);

    let predicate_true_on_first = |_: &TestKey, _: &TestValue| true;
    let result = slice.partition_point(predicate_true_on_first);
    assert_eq!(result, 3);
}

