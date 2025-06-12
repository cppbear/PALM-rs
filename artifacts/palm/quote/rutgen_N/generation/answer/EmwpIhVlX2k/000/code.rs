// Answer 0

#[derive(Default)]
struct TokenStream {
    tokens: Vec<String>,
}

trait ToTokens {
    fn to_tokens(&self, stream: &mut TokenStream);
}

impl ToTokens for String {
    fn to_tokens(&self, stream: &mut TokenStream) {
        stream.tokens.push(self.clone());
    }
}

impl TokenStream {
    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens,
    {
        for token in iter {
            token.to_tokens(self);
        }
    }
}

#[test]
fn test_append_all_empty_iter() {
    let mut stream = TokenStream::default();
    let empty_iter: Vec<String> = vec![];
    stream.append_all(empty_iter);
    assert!(stream.tokens.is_empty());
}

#[test]
fn test_append_all_single_item() {
    let mut stream = TokenStream::default();
    let single_item = String::from("token1");
    stream.append_all(vec![single_item.clone()]);
    assert_eq!(stream.tokens.len(), 1);
    assert_eq!(stream.tokens[0], single_item);
}

#[test]
fn test_append_all_multiple_items() {
    let mut stream = TokenStream::default();
    let items = vec![String::from("token1"), String::from("token2"), String::from("token3")];
    stream.append_all(items.clone());
    assert_eq!(stream.tokens.len(), 3);
    assert_eq!(stream.tokens, items);
}

