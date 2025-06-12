// Answer 0

#[derive(Debug)]
struct ThreadRng;

impl std::fmt::Display for ThreadRng {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "ThreadRng {{ .. }}")
    }
}

#[test]
fn test_thread_rng_fmt() {
    let rng = ThreadRng;
    let mut output = String::new();
    let result = {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        rng.fmt(formatter)
    };
    assert!(result.is_ok());
    assert_eq!(output, "ThreadRng { .. }");
}

#[test]
fn test_thread_rng_fmt_empty() {
    let rng = ThreadRng;
    let mut output = String::new();
    let result = {
        let formatter = &mut std::fmt::Formatter::new(&mut output);
        rng.fmt(formatter)
    };
    assert!(result.is_ok());
    assert_eq!(output, "ThreadRng { .. }");
}

