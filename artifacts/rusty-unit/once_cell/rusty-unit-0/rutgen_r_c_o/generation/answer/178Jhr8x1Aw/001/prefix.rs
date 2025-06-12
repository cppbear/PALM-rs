// Answer 0

#[test]
fn test_try_insert_integer() {
    let cell = OnceCell::new();
    let first_value = 42;
    let second_value = 17;

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
}

#[test]
fn test_try_insert_string() {
    let cell = OnceCell::new();
    let first_value = String::from("First");
    let second_value = String::from("Second");

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
} 

#[test]
fn test_try_insert_float() {
    let cell = OnceCell::new();
    let first_value = 3.14;
    let second_value = 1.59;

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
}

#[test]
fn test_try_insert_bool() {
    let cell = OnceCell::new();
    let first_value = true;
    let second_value = false;

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
} 

#[test]
fn test_try_insert_char() {
    let cell = OnceCell::new();
    let first_value = 'x';
    let second_value = 'y';

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
} 

#[test]
fn test_try_insert_vec() {
    let cell = OnceCell::new();
    let first_value = vec![1, 2, 3];
    let second_value = vec![4, 5, 6];

    let _ = cell.try_insert(first_value);
    let result = cell.try_insert(second_value);
}

