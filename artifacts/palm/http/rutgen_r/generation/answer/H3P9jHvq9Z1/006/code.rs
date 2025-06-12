// Answer 0

#[derive(Debug)]
struct MethodError;

impl std::fmt::Display for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method error")
    }
}

impl std::error::Error for MethodError {}

#[derive(Debug)]
enum ErrorKind {
    Method(MethodError),
}

struct Error {
    inner: ErrorKind,
}

impl Error {
    pub fn get_ref(&self) -> &(dyn std::error::Error + 'static) {
        use self::ErrorKind::*;

        match self.inner {
            ErrorKind::Method(ref e) => e,
        }
    }
}

#[test]
fn test_get_ref_method_error() {
    let method_error = MethodError;
    let error = Error {
        inner: ErrorKind::Method(method_error),
    };
    
    let result: &dyn std::error::Error = error.get_ref();
    assert_eq!(result.to_string(), "Method error");
}

