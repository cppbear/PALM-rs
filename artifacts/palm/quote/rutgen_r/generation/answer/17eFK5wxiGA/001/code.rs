// Answer 0

#[derive(Clone)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        TestStruct { value }
    }
    
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

#[derive(Default)]
struct TokenStream {
    tokens: Vec<TestStruct>,
}

impl TokenStream {
    fn append(&mut self, token: TestStruct) {
        self.tokens.push(token);
    }
}

#[test]
fn test_to_tokens_single_value() {
    let mut tokens = TokenStream::default();
    let test_value = TestStruct::new(42);
    test_value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.tokens.len(), 1);
    assert_eq!(tokens.tokens[0].value, 42);
}

#[test]
fn test_to_tokens_multiple_values() {
    let mut tokens = TokenStream::default();
    let test_value1 = TestStruct::new(1);
    let test_value2 = TestStruct::new(2);
    
    test_value1.to_tokens(&mut tokens);
    test_value2.to_tokens(&mut tokens);
    
    assert_eq!(tokens.tokens.len(), 2);
    assert_eq!(tokens.tokens[0].value, 1);
    assert_eq!(tokens.tokens[1].value, 2);
}

#[test]
fn test_to_tokens_empty_token_stream() {
    let mut tokens = TokenStream::default();
    let test_value = TestStruct::new(0);
    
    test_value.to_tokens(&mut tokens);
    
    assert_eq!(tokens.tokens.len(), 1);
    assert_eq!(tokens.tokens[0].value, 0);
}

