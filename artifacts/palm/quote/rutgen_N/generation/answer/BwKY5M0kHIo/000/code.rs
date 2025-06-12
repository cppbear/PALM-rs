// Answer 0

#[derive(Debug)]
struct TokenTree;

struct TokenVec(Vec<TokenTree>);

impl TokenVec {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.0.extend(std::iter::once(token.into()));
    }
}

impl From<u32> for TokenTree {
    fn from(_: u32) -> Self {
        TokenTree
    }
}

#[test]
fn test_append_empty() {
    let mut tokens = TokenVec(Vec::new());
    tokens.append(1u32);
    assert_eq!(tokens.0.len(), 1);
}

#[test]
fn test_append_multiple() {
    let mut tokens = TokenVec(Vec::new());
    tokens.append(1u32);
    tokens.append(2u32);
    tokens.append(3u32);
    assert_eq!(tokens.0.len(), 3);
}

#[test]
fn test_append_with_non_standard_type() {
    #[derive(Debug)]
    struct CustomToken(u32);

    impl Into<TokenTree> for CustomToken {
        fn into(self) -> TokenTree {
            TokenTree
        }
    }

    let mut tokens = TokenVec(Vec::new());
    tokens.append(CustomToken(5));
    assert_eq!(tokens.0.len(), 1);
}

