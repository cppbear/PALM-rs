// Answer 0

#[test]
fn test_try_insert_existing_value() {
    let cell = OnceCell::with_value(42);
    let res = cell.try_insert(43);
}

#[test]
fn test_try_insert_same_value() {
    let cell = OnceCell::with_value(100);
    let res = cell.try_insert(100);
}

#[test]
fn test_try_insert_different_value() {
    let cell = OnceCell::with_value(123);
    let res = cell.try_insert(124);
}

#[test]
fn test_try_insert_large_value() {
    let cell = OnceCell::with_value(1_000_000);
    let res = cell.try_insert(1_000_001);
}

#[test]
fn test_try_insert_negative_value() {
    let cell = OnceCell::with_value(-10);
    let res = cell.try_insert(-9);
}

#[test]
fn test_try_insert_zero() {
    let cell = OnceCell::with_value(0);
    let res = cell.try_insert(1);
}

