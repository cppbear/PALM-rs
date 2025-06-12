// Answer 0

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn custom(msg: &str) -> Self {
        Error {
            message: msg.to_string(),
        }
    }
}

#[derive(Debug)]
struct MyStruct;

#[test]
fn test_deserialize_other() {
    let result: Result<MyStruct, Error> = deserialize_other();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().message, "can only flatten structs and maps");
}

