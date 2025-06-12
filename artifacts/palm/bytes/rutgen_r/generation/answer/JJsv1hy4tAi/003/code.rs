// Answer 0

#[test]
fn test_fmt_empty() {
    struct Dummy<'a>(&'a [u8]);

    let data = Dummy(&[]);
    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(buffer, "");
}

#[test]
fn test_fmt_single_byte_zero() {
    struct Dummy<'a>(&'a [u8]);

    let data = Dummy(&[0]);
    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(buffer, "00");
}

#[test]
fn test_fmt_single_byte_max() {
    struct Dummy<'a>(&'a [u8]);

    let data = Dummy(&[255]);
    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(buffer, "ff");
}

#[test]
fn test_fmt_multiple_bytes() {
    struct Dummy<'a>(&'a [u8]);

    let data = Dummy(&[1, 2, 3, 255]);
    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| data.fmt(f));
    assert!(result.is_ok());
    assert_eq!(buffer, "010203ff");
}

