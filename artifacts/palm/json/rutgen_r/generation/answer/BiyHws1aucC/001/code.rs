// Answer 0

#[derive(Debug)]
struct Number(i32);

impl std::fmt::Display for Number {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Number({})", self.0)
    }
}

#[test]
fn test_number_fmt_positive() {
    let num = Number(42);
    let mut output = String::new();
    let result = write!(&mut output, "{}", num);
    assert!(result.is_ok());
    assert_eq!(output, "Number(42)");
}

#[test]
fn test_number_fmt_negative() {
    let num = Number(-1);
    let mut output = String::new();
    let result = write!(&mut output, "{}", num);
    assert!(result.is_ok());
    assert_eq!(output, "Number(-1)");
}

#[test]
fn test_number_fmt_zero() {
    let num = Number(0);
    let mut output = String::new();
    let result = write!(&mut output, "{}", num);
    assert!(result.is_ok());
    assert_eq!(output, "Number(0)");
}

#[test]
fn test_number_fmt_large() {
    let num = Number(1_000_000);
    let mut output = String::new();
    let result = write!(&mut output, "{}", num);
    assert!(result.is_ok());
    assert_eq!(output, "Number(1000000)");
}

#[test]
fn test_number_fmt_small() {
    let num = Number(-1_000_000);
    let mut output = String::new();
    let result = write!(&mut output, "{}", num);
    assert!(result.is_ok());
    assert_eq!(output, "Number(-1000000)");
}

