pub fn push_group(tokens: &mut TokenStream, delimiter: Delimiter, inner: TokenStream) {
    tokens.append(Group::new(delimiter, inner));
}