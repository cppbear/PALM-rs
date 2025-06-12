// Answer 0

#[derive(Debug)]
struct TestEngine;

impl Engine for TestEngine {
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        base64::encode(input)
    }
}

#[test]
fn test_encode_engine_with_empty_input() {
    let engine = TestEngine;
    let result = encode_engine("", &engine);
    assert_eq!(result, "");
}

#[test]
fn test_encode_engine_with_simple_input() {
    let engine = TestEngine;
    let result = encode_engine("hello", &engine);
    assert_eq!(result, "aGVsbG8=");
}

#[test]
fn test_encode_engine_with_special_characters() {
    let engine = TestEngine;
    let result = encode_engine("!@#$%^&*()", &engine);
    assert_eq!(result, "IUAjJCVeJiaoKQ==");
}

#[test]
fn test_encode_engine_with_binary_data() {
    let engine = TestEngine;
    let binary_data: &[u8] = &[0, 159, 146, 150];
    let result = encode_engine(binary_data, &engine);
    assert_eq!(result, "APYQ");
}

