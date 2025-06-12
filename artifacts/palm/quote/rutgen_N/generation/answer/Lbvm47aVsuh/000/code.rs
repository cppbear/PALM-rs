// Answer 0

#[derive(Clone)]
struct Example;

impl Example {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(self.clone()));
    }
}

#[test]
fn test_to_tokens_single_instance() {
    let mut tokens = TokenStream::new();
    let example = Example;
    example.to_tokens(&mut tokens);
    assert_eq!(tokens.to_string(), example.to_string());
}

#[test]
fn test_to_tokens_multiple_instances() {
    let mut tokens = TokenStream::new();
    let example1 = Example;
    let example2 = Example;
    
    example1.to_tokens(&mut tokens);
    example2.to_tokens(&mut tokens);
    
    assert_eq!(tokens.to_string(), format!("{}{}", example1.to_string(), example2.to_string()));
}

