// Answer 0

#[test]
fn test_method_with_valid_string() {
    let builder = Builder::new();
    let result = builder.method("POST").body(()).unwrap();
    assert_eq!(result.method_ref().unwrap().as_str(), "POST");
}

#[test]
fn test_method_with_invalid_string() {
    #[derive(Debug)]
    struct InvalidMethod;
    
    impl TryInto<Method> for InvalidMethod {
        type Error = crate::Error;

        fn try_into(self) -> Result<Method> {
            Err(self.into())
        }
    }
    
    let builder = Builder::new();
    let result = builder.method(InvalidMethod);
    assert!(result.inner.is_err());
}

#[test]
fn test_method_with_get_default() {
    let builder = Builder::new();
    let result = builder.body(()).unwrap();
    assert_eq!(result.method_ref().unwrap().as_str(), "GET");
}

#[test]
fn test_method_with_special_characters() {
    let builder = Builder::new();
    let result = builder.method("DELETE!").body(()).unwrap();
    assert_eq!(result.method_ref().unwrap().as_str(), "DELETE!");
}

#[test]
#[should_panic]
fn test_method_with_empty_string() {
    let builder = Builder::new();
    let _result = builder.method("").body(()).unwrap();
}

#[test]
fn test_method_with_uppercase() {
    let builder = Builder::new();
    let result = builder.method("PATCH").body(()).unwrap();
    assert_eq!(result.method_ref().unwrap().as_str(), "PATCH");
}

