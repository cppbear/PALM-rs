// Answer 0

#[test]
fn test_type_fmt_bool_true() {
    let value = Value::Bool(true);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    type_instance.fmt(&mut formatter);
}

#[test]
fn test_type_fmt_bool_false() {
    let value = Value::Bool(false);
    let type_instance = Type(&value);
    let mut formatter = fmt::Formatter::new();
    type_instance.fmt(&mut formatter);
}

