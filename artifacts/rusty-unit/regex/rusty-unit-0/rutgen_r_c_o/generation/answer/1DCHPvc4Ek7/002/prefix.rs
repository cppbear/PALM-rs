// Answer 0

#[test]
fn test_fmt_compiled_too_big_zero() {
    let error = Error::CompiledTooBig(0);
    let mut formatter = String::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_compiled_too_big_small_value() {
    let error = Error::CompiledTooBig(1);
    let mut formatter = String::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_compiled_too_big_mid_value() {
    let error = Error::CompiledTooBig(2147483647);
    let mut formatter = String::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_compiled_too_big_max_value() {
    let error = Error::CompiledTooBig(4294967295);
    let mut formatter = String::new();
    let _ = error.fmt(&mut formatter);
}

