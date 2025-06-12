// Answer 0

#[test]
fn test_position_of_index_first_line() {
    let data = b"Hello, World!\nWelcome to Rust.\n";
    let reader = SliceRead::new(data);

    let pos = reader.position_of_index(5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_position_of_index_newline() {
    let data = b"Hello, World!\nWelcome to Rust.\n";
    let reader = SliceRead::new(data);

    let pos = reader.position_of_index(13);
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_position_of_index_mid_line() {
    let data = b"Hello, World!\nWelcome to Rust.\n";
    let reader = SliceRead::new(data);

    let pos = reader.position_of_index(15);
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 3);
}

#[test]
fn test_position_of_index_no_newline() {
    let data = b"HelloWorld";
    let reader = SliceRead::new(data);

    let pos = reader.position_of_index(5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_position_of_index_empty_slice() {
    let data: &[u8] = b"";
    let reader = SliceRead::new(data);

    let pos = reader.position_of_index(0);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);
}

