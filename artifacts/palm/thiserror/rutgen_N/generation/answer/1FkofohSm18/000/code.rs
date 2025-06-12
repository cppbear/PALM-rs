// Answer 0

#[derive(Debug)]
struct MyError;

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError occurred")
    }
}

impl std::error::Error for MyError {}

impl AsRef<dyn std::error::Error> for MyError {
    fn as_ref(&self) -> &(dyn std::error::Error + 'static) {
        self
    }
}

#[test]
fn test_as_dyn_error() {
    let my_error = MyError;
    let dyn_error: &(dyn std::error::Error + 'static) = my_error.as_ref().as_dyn_error();
    assert_eq!(dyn_error.to_string(), "MyError occurred");
}

