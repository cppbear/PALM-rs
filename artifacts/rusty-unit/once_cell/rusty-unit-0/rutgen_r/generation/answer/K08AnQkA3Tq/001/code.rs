// Answer 0

#[test]
fn test_lazy_new_with_string_uppercase() {
    use once_cell::unsync::Lazy;

    let hello = "Hello, World!".to_string();

    let lazy = Lazy::new(|| hello.to_uppercase());

    assert_eq!(&*lazy, "HELLO, WORLD!");
}

#[test]
fn test_lazy_new_with_empty_string() {
    use once_cell::unsync::Lazy;

    let empty = "".to_string();

    let lazy = Lazy::new(|| empty.to_uppercase());

    assert_eq!(&*lazy, "");
}

#[test]
fn test_lazy_new_with_numeric_string() {
    use once_cell::unsync::Lazy;

    let num_str = "12345".to_string();

    let lazy = Lazy::new(|| num_str.to_uppercase());

    assert_eq!(&*lazy, "12345");
}

#[test]
fn test_lazy_new_with_special_characters() {
    use once_cell::unsync::Lazy;

    let special_str = "!@#$%&*()".to_string();

    let lazy = Lazy::new(|| special_str.to_uppercase());

    assert_eq!(&*lazy, "!@#$%&*()");
}

#[test]
fn test_lazy_new_with_long_string() {
    use once_cell::unsync::Lazy;

    let long_str = "a".repeat(1000); // Create a long string with 1000 'a's

    let lazy = Lazy::new(|| long_str.to_uppercase());

    assert_eq!(&*lazy, "A".repeat(1000));
}

