// Answer 0

#[test]
fn test_compiled_too_big_zero() {
    let error = Error::CompiledTooBig(0);
    let mut buf = vec![];
    let _ = error.fmt(&mut fmt::Formatter::new(&mut buf));
}

#[test]
fn test_compiled_too_big_small() {
    let error = Error::CompiledTooBig(1);
    let mut buf = vec![];
    let _ = error.fmt(&mut fmt::Formatter::new(&mut buf));
}

#[test]
fn test_compiled_too_big_large() {
    let error = Error::CompiledTooBig(usize::MAX);
    let mut buf = vec![];
    let _ = error.fmt(&mut fmt::Formatter::new(&mut buf));
}

#[test]
fn test_compiled_too_big_mid_range() {
    let error = Error::CompiledTooBig(100);
    let mut buf = vec![];
    let _ = error.fmt(&mut fmt::Formatter::new(&mut buf));
}

