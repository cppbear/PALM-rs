// Answer 0

#[test]
fn test_as_bytes() {
    struct MyStruct<'a>(&'a [u8]);

    let my_struct = MyStruct(&[1, 2, 3, 4]);
    let result: &[u8] = my_struct.as_bytes();
    assert_eq!(result, &[1, 2, 3, 4]);
}

#[test]
fn test_as_bytes_empty() {
    struct MyStruct<'a>(&'a [u8]);

    let my_struct = MyStruct(&[]);
    let result: &[u8] = my_struct.as_bytes();
    assert_eq!(result, &[]);
}

