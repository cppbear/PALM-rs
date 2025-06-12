// Answer 0

#[test]
fn test_fmt_with_newline() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'\n']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\n""#);
}

#[test]
fn test_fmt_with_carriage_return() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'\r']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\r""#);
}

#[test]
fn test_fmt_with_tab() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'\t']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\t""#);
}

#[test]
fn test_fmt_with_backslash() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'\\']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\\""#);
}

#[test]
fn test_fmt_with_double_quote() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'"']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\""#);
}

#[test]
fn test_fmt_with_null() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'\0']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\0""#);
}

#[test]
fn test_fmt_with_non_printable() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[0x01]);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"\x01""#);
}

#[test]
fn test_fmt_with_multiple_bytes() {
    struct TestStruct<'a>(&'a [u8]);
    let test_data = TestStruct(&[b'a', b'\n', b'b', b'\r']);
    let mut output = String::new();
    let result = write!(&mut output, "b\"").and_then(|_| test_data.fmt(&mut output));
    assert_eq!(result, Ok(()));
    assert_eq!(output, r#"b"ab\nb\r""#);
}

