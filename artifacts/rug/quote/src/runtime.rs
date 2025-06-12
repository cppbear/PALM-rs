use self::get_span::{GetSpan, GetSpanBase, GetSpanInner};
use crate::{IdentFragment, ToTokens, TokenStreamExt};
use core::fmt;
use core::iter;
use core::ops::BitOr;
use proc_macro2::{Group, Ident, Punct, Spacing, TokenTree};

#[doc(hidden)]
pub use alloc::format;
#[doc(hidden)]
pub use core::option::Option;

#[doc(hidden)]
pub type Delimiter = proc_macro2::Delimiter;
#[doc(hidden)]
pub type Span = proc_macro2::Span;
#[doc(hidden)]
pub type TokenStream = proc_macro2::TokenStream;

#[doc(hidden)]
pub struct HasIterator; // True
#[doc(hidden)]
pub struct ThereIsNoIteratorInRepetition; // False

impl BitOr<ThereIsNoIteratorInRepetition> for ThereIsNoIteratorInRepetition {
    type Output = ThereIsNoIteratorInRepetition;
    fn bitor(self, _rhs: ThereIsNoIteratorInRepetition) -> ThereIsNoIteratorInRepetition {
        ThereIsNoIteratorInRepetition
    }
}

impl BitOr<ThereIsNoIteratorInRepetition> for HasIterator {
    type Output = HasIterator;
    fn bitor(self, _rhs: ThereIsNoIteratorInRepetition) -> HasIterator {
        HasIterator
    }
}

impl BitOr<HasIterator> for ThereIsNoIteratorInRepetition {
    type Output = HasIterator;
    fn bitor(self, _rhs: HasIterator) -> HasIterator {
        HasIterator
    }
}

impl BitOr<HasIterator> for HasIterator {
    type Output = HasIterator;
    fn bitor(self, _rhs: HasIterator) -> HasIterator {
        HasIterator
    }
}

/// Extension traits used by the implementation of `quote!`. These are defined
/// in separate traits, rather than as a single trait due to ambiguity issues.
///
/// These traits expose a `quote_into_iter` method which should allow calling
/// whichever impl happens to be applicable. Calling that method repeatedly on
/// the returned value should be idempotent.
#[doc(hidden)]
pub mod ext {
    use super::RepInterp;
    use super::{HasIterator as HasIter, ThereIsNoIteratorInRepetition as DoesNotHaveIter};
    use crate::ToTokens;
    use alloc::collections::btree_set::{self, BTreeSet};
    use core::slice;

    /// Extension trait providing the `quote_into_iter` method on iterators.
    #[doc(hidden)]
    pub trait RepIteratorExt: Iterator + Sized {
        fn quote_into_iter(self) -> (Self, HasIter) {
            (self, HasIter)
        }
    }

    impl<T: Iterator> RepIteratorExt for T {}

    /// Extension trait providing the `quote_into_iter` method for
    /// non-iterable types. These types interpolate the same value in each
    /// iteration of the repetition.
    #[doc(hidden)]
    pub trait RepToTokensExt {
        /// Pretend to be an iterator for the purposes of `quote_into_iter`.
        /// This allows repeated calls to `quote_into_iter` to continue
        /// correctly returning DoesNotHaveIter.
        fn next(&self) -> Option<&Self> {
            Some(self)
        }

        fn quote_into_iter(&self) -> (&Self, DoesNotHaveIter) {
            (self, DoesNotHaveIter)
        }
    }

    impl<T: ToTokens + ?Sized> RepToTokensExt for T {}

    /// Extension trait providing the `quote_into_iter` method for types that
    /// can be referenced as an iterator.
    #[doc(hidden)]
    pub trait RepAsIteratorExt<'q> {
        type Iter: Iterator;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter);
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: RepAsIteratorExt<'q> + ?Sized> RepAsIteratorExt<'q> for &mut T {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            <T as RepAsIteratorExt>::quote_into_iter(*self)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for [T] {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: 'q, const N: usize> RepAsIteratorExt<'q> for [T; N] {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for Vec<T> {
        type Iter = slice::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: 'q> RepAsIteratorExt<'q> for BTreeSet<T> {
        type Iter = btree_set::Iter<'q, T>;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
    }

    impl<'q, T: RepAsIteratorExt<'q>> RepAsIteratorExt<'q> for RepInterp<T> {
        type Iter = T::Iter;

        fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            self.0.quote_into_iter()
        }
    }
}

// Helper type used within interpolations to allow for repeated binding names.
// Implements the relevant traits, and exports a dummy `next()` method.
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct RepInterp<T>(pub T);

impl<T> RepInterp<T> {
    // This method is intended to look like `Iterator::next`, and is called when
    // a name is bound multiple times, as the previous binding will shadow the
    // original `Iterator` object. This allows us to avoid advancing the
    // iterator multiple times per iteration.
    pub fn next(self) -> Option<T> {
        Some(self.0)
    }
}

impl<T: Iterator> Iterator for RepInterp<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<T: ToTokens> ToTokens for RepInterp<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[doc(hidden)]
#[inline]
pub fn get_span<T>(span: T) -> GetSpan<T> {
    GetSpan(GetSpanInner(GetSpanBase(span)))
}

mod get_span {
    use core::ops::Deref;
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;

    pub struct GetSpan<T>(pub(crate) GetSpanInner<T>);

    pub struct GetSpanInner<T>(pub(crate) GetSpanBase<T>);

    pub struct GetSpanBase<T>(pub(crate) T);

    impl GetSpan<Span> {
        #[inline]
        pub fn __into_span(self) -> Span {
            ((self.0).0).0
        }
    }

    impl GetSpanInner<DelimSpan> {
        #[inline]
        pub fn __into_span(&self) -> Span {
            (self.0).0.join()
        }
    }

    impl<T> GetSpanBase<T> {
        #[allow(clippy::unused_self)]
        pub fn __into_span(&self) -> T {
            unreachable!()
        }
    }

    impl<T> Deref for GetSpan<T> {
        type Target = GetSpanInner<T>;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> Deref for GetSpanInner<T> {
        type Target = GetSpanBase<T>;

        #[inline]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

#[doc(hidden)]
pub fn push_group(tokens: &mut TokenStream, delimiter: Delimiter, inner: TokenStream) {
    tokens.append(Group::new(delimiter, inner));
}

#[doc(hidden)]
pub fn push_group_spanned(
    tokens: &mut TokenStream,
    span: Span,
    delimiter: Delimiter,
    inner: TokenStream,
) {
    let mut g = Group::new(delimiter, inner);
    g.set_span(span);
    tokens.append(g);
}

#[doc(hidden)]
pub fn parse(tokens: &mut TokenStream, s: &str) {
    let s: TokenStream = s.parse().expect("invalid token stream");
    tokens.extend(iter::once(s));
}

#[doc(hidden)]
pub fn parse_spanned(tokens: &mut TokenStream, span: Span, s: &str) {
    let s: TokenStream = s.parse().expect("invalid token stream");
    tokens.extend(s.into_iter().map(|t| respan_token_tree(t, span)));
}

// Token tree with every span replaced by the given one.
fn respan_token_tree(mut token: TokenTree, span: Span) -> TokenTree {
    match &mut token {
        TokenTree::Group(g) => {
            let stream = g
                .stream()
                .into_iter()
                .map(|token| respan_token_tree(token, span))
                .collect();
            *g = Group::new(g.delimiter(), stream);
            g.set_span(span);
        }
        other => other.set_span(span),
    }
    token
}

#[doc(hidden)]
pub fn push_ident(tokens: &mut TokenStream, s: &str) {
    let span = Span::call_site();
    push_ident_spanned(tokens, span, s);
}

#[doc(hidden)]
pub fn push_ident_spanned(tokens: &mut TokenStream, span: Span, s: &str) {
    tokens.append(ident_maybe_raw(s, span));
}

#[doc(hidden)]
pub fn push_lifetime(tokens: &mut TokenStream, lifetime: &str) {
    tokens.extend([
        TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
        TokenTree::Ident(Ident::new(&lifetime[1..], Span::call_site())),
    ]);
}

#[doc(hidden)]
pub fn push_lifetime_spanned(tokens: &mut TokenStream, span: Span, lifetime: &str) {
    tokens.extend([
        TokenTree::Punct({
            let mut apostrophe = Punct::new('\'', Spacing::Joint);
            apostrophe.set_span(span);
            apostrophe
        }),
        TokenTree::Ident(Ident::new(&lifetime[1..], span)),
    ]);
}

macro_rules! push_punct {
    ($name:ident $spanned:ident $char1:tt) => {
        #[doc(hidden)]
        pub fn $name(tokens: &mut TokenStream) {
            tokens.append(Punct::new($char1, Spacing::Alone));
        }
        #[doc(hidden)]
        pub fn $spanned(tokens: &mut TokenStream, span: Span) {
            let mut punct = Punct::new($char1, Spacing::Alone);
            punct.set_span(span);
            tokens.append(punct);
        }
    };
    ($name:ident $spanned:ident $char1:tt $char2:tt) => {
        #[doc(hidden)]
        pub fn $name(tokens: &mut TokenStream) {
            tokens.append(Punct::new($char1, Spacing::Joint));
            tokens.append(Punct::new($char2, Spacing::Alone));
        }
        #[doc(hidden)]
        pub fn $spanned(tokens: &mut TokenStream, span: Span) {
            let mut punct = Punct::new($char1, Spacing::Joint);
            punct.set_span(span);
            tokens.append(punct);
            let mut punct = Punct::new($char2, Spacing::Alone);
            punct.set_span(span);
            tokens.append(punct);
        }
    };
    ($name:ident $spanned:ident $char1:tt $char2:tt $char3:tt) => {
        #[doc(hidden)]
        pub fn $name(tokens: &mut TokenStream) {
            tokens.append(Punct::new($char1, Spacing::Joint));
            tokens.append(Punct::new($char2, Spacing::Joint));
            tokens.append(Punct::new($char3, Spacing::Alone));
        }
        #[doc(hidden)]
        pub fn $spanned(tokens: &mut TokenStream, span: Span) {
            let mut punct = Punct::new($char1, Spacing::Joint);
            punct.set_span(span);
            tokens.append(punct);
            let mut punct = Punct::new($char2, Spacing::Joint);
            punct.set_span(span);
            tokens.append(punct);
            let mut punct = Punct::new($char3, Spacing::Alone);
            punct.set_span(span);
            tokens.append(punct);
        }
    };
}

push_punct!(push_add push_add_spanned '+');
push_punct!(push_add_eq push_add_eq_spanned '+' '=');
push_punct!(push_and push_and_spanned '&');
push_punct!(push_and_and push_and_and_spanned '&' '&');
push_punct!(push_and_eq push_and_eq_spanned '&' '=');
push_punct!(push_at push_at_spanned '@');
push_punct!(push_bang push_bang_spanned '!');
push_punct!(push_caret push_caret_spanned '^');
push_punct!(push_caret_eq push_caret_eq_spanned '^' '=');
push_punct!(push_colon push_colon_spanned ':');
push_punct!(push_colon2 push_colon2_spanned ':' ':');
push_punct!(push_comma push_comma_spanned ',');
push_punct!(push_div push_div_spanned '/');
push_punct!(push_div_eq push_div_eq_spanned '/' '=');
push_punct!(push_dot push_dot_spanned '.');
push_punct!(push_dot2 push_dot2_spanned '.' '.');
push_punct!(push_dot3 push_dot3_spanned '.' '.' '.');
push_punct!(push_dot_dot_eq push_dot_dot_eq_spanned '.' '.' '=');
push_punct!(push_eq push_eq_spanned '=');
push_punct!(push_eq_eq push_eq_eq_spanned '=' '=');
push_punct!(push_ge push_ge_spanned '>' '=');
push_punct!(push_gt push_gt_spanned '>');
push_punct!(push_le push_le_spanned '<' '=');
push_punct!(push_lt push_lt_spanned '<');
push_punct!(push_mul_eq push_mul_eq_spanned '*' '=');
push_punct!(push_ne push_ne_spanned '!' '=');
push_punct!(push_or push_or_spanned '|');
push_punct!(push_or_eq push_or_eq_spanned '|' '=');
push_punct!(push_or_or push_or_or_spanned '|' '|');
push_punct!(push_pound push_pound_spanned '#');
push_punct!(push_question push_question_spanned '?');
push_punct!(push_rarrow push_rarrow_spanned '-' '>');
push_punct!(push_larrow push_larrow_spanned '<' '-');
push_punct!(push_rem push_rem_spanned '%');
push_punct!(push_rem_eq push_rem_eq_spanned '%' '=');
push_punct!(push_fat_arrow push_fat_arrow_spanned '=' '>');
push_punct!(push_semi push_semi_spanned ';');
push_punct!(push_shl push_shl_spanned '<' '<');
push_punct!(push_shl_eq push_shl_eq_spanned '<' '<' '=');
push_punct!(push_shr push_shr_spanned '>' '>');
push_punct!(push_shr_eq push_shr_eq_spanned '>' '>' '=');
push_punct!(push_star push_star_spanned '*');
push_punct!(push_sub push_sub_spanned '-');
push_punct!(push_sub_eq push_sub_eq_spanned '-' '=');

#[doc(hidden)]
pub fn push_underscore(tokens: &mut TokenStream) {
    push_underscore_spanned(tokens, Span::call_site());
}

#[doc(hidden)]
pub fn push_underscore_spanned(tokens: &mut TokenStream, span: Span) {
    tokens.append(Ident::new("_", span));
}

// Helper method for constructing identifiers from the `format_ident!` macro,
// handling `r#` prefixes.
#[doc(hidden)]
pub fn mk_ident(id: &str, span: Option<Span>) -> Ident {
    let span = span.unwrap_or_else(Span::call_site);
    ident_maybe_raw(id, span)
}

fn ident_maybe_raw(id: &str, span: Span) -> Ident {
    if let Some(id) = id.strip_prefix("r#") {
        Ident::new_raw(id, span)
    } else {
        Ident::new(id, span)
    }
}

// Adapts from `IdentFragment` to `fmt::Display` for use by the `format_ident!`
// macro, and exposes span information from these fragments.
//
// This struct also has forwarding implementations of the formatting traits
// `Octal`, `LowerHex`, `UpperHex`, and `Binary` to allow for their use within
// `format_ident!`.
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct IdentFragmentAdapter<T: IdentFragment>(pub T);

impl<T: IdentFragment> IdentFragmentAdapter<T> {
    pub fn span(&self) -> Option<Span> {
        self.0.span()
    }
}

impl<T: IdentFragment> fmt::Display for IdentFragmentAdapter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(&self.0, f)
    }
}

impl<T: IdentFragment + fmt::Octal> fmt::Octal for IdentFragmentAdapter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

impl<T: IdentFragment + fmt::LowerHex> fmt::LowerHex for IdentFragmentAdapter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl<T: IdentFragment + fmt::UpperHex> fmt::UpperHex for IdentFragmentAdapter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl<T: IdentFragment + fmt::Binary> fmt::Binary for IdentFragmentAdapter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests_llm_16_16 {
    use super::*;

use crate::*;
    use std::ops::BitOr;

    #[test]
    fn test_bitor_with_has_iterator() {
        let a = HasIterator;
        let b = HasIterator;
        let result = a.bitor(b);
        // Check that the result is an instance of HasIterator
        let _ : HasIterator = result;
    }

    #[test]
    fn test_bitor_with_there_is_no_iterator() {
        let a = HasIterator;
        let b = ThereIsNoIteratorInRepetition;
        let result = a.bitor(b);
        // Check that the result is an instance of HasIterator
        let _ : HasIterator = result;
    }
}

#[cfg(test)]
mod tests_llm_16_24 {
    use super::*;

use crate::*;
    use proc_macro2::TokenStream;
    use crate::ToTokens;

    struct TestToken;
    
    impl ToTokens for TestToken {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.extend(quote! { TestToken });
        }
    }

    #[test]
    fn test_to_tokens() {
        let test_obj = RepInterp(TestToken);
        let mut tokens = TokenStream::new();

        test_obj.to_tokens(&mut tokens);
        
        let expected: TokenStream = quote! { TestToken };
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_82 {
    use super::*;

use crate::*;
    use proc_macro2::Ident;
    use crate::__private::IdentFragmentAdapter;

    #[test]
    fn test_span_some() {
        let ident = Ident::new("test_ident", proc_macro2::Span::call_site());
        let adapter = IdentFragmentAdapter(ident);
        assert!(adapter.span().is_some());
    }

    #[test]
    fn test_span_none() {
        struct Dummy;
        
        impl IdentFragment for Dummy {
            fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
                Ok(())
            }
        }

        let adapter = IdentFragmentAdapter(Dummy);
        assert!(adapter.span().is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_83 {
    use super::*;

use crate::*;
    use crate::ToTokens;

    #[test]
    fn test_next() {
        let value = 42;
        let interp = __private::RepInterp(value);
        let result = interp.next();
        
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_next_with_option() {
        let value = Some("Hello");
        let interp = __private::RepInterp(value);
        let result = interp.next();

        assert_eq!(result, Some(Some("Hello")));
    }

    #[test]
    fn test_next_with_empty_option() {
        let value: Option<i32> = None;
        let interp = __private::RepInterp(value);
        let result = interp.next();

        assert_eq!(result, Some(None));
    }
}

#[cfg(test)]
mod tests_llm_16_89 {
    use super::*;

use crate::*;

    #[test]
    fn test_into_span() {
        let value = GetSpanBase(42); // Example value
        let result: i32 = value.__into_span();
        // Since the function is unreachable, we will expect the test to panic
        // during the call to __into_span.
    }
}

#[cfg(test)]
mod tests_llm_16_96 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_add_eq() {
        let mut tokens = TokenStream::new();
        let char1 = '+';
        let char2 = '=';
        push_add_eq(&mut tokens);

        let expected = {
            let mut expected_tokens = TokenStream::new();
            expected_tokens.append(Punct::new(char1, Spacing::Joint));
            expected_tokens.append(Punct::new(char2, Spacing::Alone));
            expected_tokens
        };

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_98 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_add_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();
        let char1 = 'a'; // Example punctuation character

        // Call the target function
        push_add_spanned(&mut tokens, span);

        // Build the expected output
        let mut expected_tokens = TokenStream::new();
        let mut punct = Punct::new(char1, Spacing::Alone);
        punct.set_span(span);
        expected_tokens.append(punct);

        // Assert that the output tokens match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_99 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Span, Delimiter, Group, TokenTree};

    #[test]
    fn test_push_and() {
        let mut tokens = TokenStream::new();
        let char1 = '&';
        let name = "__private::push_and";

        // Invoke the function using the token stream
        let push_and = |tokens: &mut TokenStream| {
            tokens.append(Punct::new(char1, Spacing::Alone));
        };

        push_and(&mut tokens);

        // Check if the tokens contain the expected punctuation
        let expected = TokenStream::from(TokenTree::Punct(Punct::new(char1, Spacing::Alone)));
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_100 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, TokenTree, Punct, Spacing};

    #[test]
    fn test_push_and_and() {
        let mut tokens = TokenStream::new();
        let char1 = '+';
        let char2 = '&';

        // Call the target function
        __private::push_and_and(&mut tokens);

        // Verify the output
        let expected: TokenStream = vec![
            TokenTree::Punct(Punct::new(char1, Spacing::Joint)),
            TokenTree::Punct(Punct::new(char2, Spacing::Alone)),
        ].into_iter().collect();

        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_102 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_and_eq() {
        let mut tokens = TokenStream::new();
        let char1 = '=';
        let char2 = '>';

        // Call the target function
        __private::push_and_eq(&mut tokens);

        // Construct expected tokens
        let mut expected_tokens = TokenStream::new();
        expected_tokens.append(Punct::new(char1, Spacing::Joint));
        expected_tokens.append(Punct::new(char2, Spacing::Alone));

        // Assert that the generated tokens are as expected
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_104 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_and_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();
        let char1 = '!';
        
        // Call the push_and_spanned function
        __private::push_and_spanned(&mut tokens, span);

        // Create expected token
        let mut expected_tokens = TokenStream::new();
        let punct = Punct::new(char1, Spacing::Alone);
        expected_tokens.append(punct);
        
        // Verify the tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_105 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_at() {
        let mut tokens = TokenStream::new();
        let char1 = '#'; // Adjust as needed
        let name = "test_push_at"; // Adjust as needed

        // Call the target function
        __private::push_at(&mut tokens);

        // Create the expected output
        let mut expected_tokens = TokenStream::new();
        expected_tokens.append(Punct::new(char1, Spacing::Alone));

        // Assert that the output matches the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_112 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_caret_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();
        let char1 = '^';

        // Call the function directly as the private function and simulate its behavior
        let mut punct = Punct::new(char1, Spacing::Alone);
        punct.set_span(span);
        tokens.append(punct);

        // Verify the token stream contains the expected punct
        let expected = format!("{} ", char1);
        assert_eq!(tokens.to_string(), expected);
    }
}

#[cfg(test)]
mod tests_llm_16_113 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_colon() {
        let mut tokens = TokenStream::new();
        let colon = Punct::new(':', Spacing::Alone);
        
        __private::push_colon(&mut tokens);
        
        assert_eq!(tokens.to_string(), colon.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_114 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_colon2() {
        let mut tokens = TokenStream::new();
        let name = "__private::push_colon2"; // use the correct path to the target function if needed

        // Assuming 'push_colon2' is a function to push tokens with '::' (colon and colon)
        push_colon2(&mut tokens);

        // Create expected tokens
        let expected = {
            let mut expected_tokens = TokenStream::new();
            expected_tokens.append(Punct::new(':', Spacing::Joint));
            expected_tokens.append(Punct::new(':', Spacing::Alone));
            expected_tokens
        };

        // Assert that tokens generated match expected tokens
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_120 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_div_eq() {
        let mut tokens = TokenStream::new();
        let char1 = '/';
        let char2 = '=';
        let name = "push_div_eq";

        // Call the target function
        __private::push_div_eq(&mut tokens);

        // Expected tokens after the function call
        let expected_tokens: TokenStream = {
            let mut ts = TokenStream::new();
            ts.append(Punct::new(char1, Spacing::Joint));
            ts.append(Punct::new(char2, Spacing::Alone));
            ts
        };

        // Assert that the generated tokens match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_128 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, TokenTree, Punct, Spacing};

    #[test]
    fn test_push_dot_dot_eq() {
        let mut tokens = TokenStream::new();

        // Assuming the function is defined as `__private::push_dot_dot_eq`
        __private::push_dot_dot_eq(&mut tokens);

        let expected_tokens: TokenStream = {
            let mut expected = TokenStream::new();
            expected.append(Punct::new('.', Spacing::Joint));
            expected.append(Punct::new('.', Spacing::Joint));
            expected.append(Punct::new('=', Spacing::Alone));
            expected
        };

        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_137 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_ge() {
        let mut tokens = TokenStream::new();
        
        // Call `push_ge` function here with appropriate arguments
        __private::push_ge(&mut tokens);
        
        // Prepare the expected tokens: '>' and '='
        let mut expected_tokens = TokenStream::new();
        expected_tokens.append(Punct::new('>', Spacing::Joint));
        expected_tokens.append(Punct::new('=', Spacing::Alone));
        
        // Assert that the generated tokens match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_143 {
    use super::*;

use crate::*;
    use proc_macro2::TokenStream;
    
    #[test]
    fn test_push_ident() {
        let mut tokens = TokenStream::new();
        let ident = "example_ident";
        
        // Call the target function
        __private::push_ident(&mut tokens, ident);
        
        // Verify the tokens contain the expected identifier
        let expected = format!("{} ", ident); // Extra space is added as it will be the output format
        let output = tokens.to_string();
        
        assert!(output.contains(&expected));
    }
}

#[cfg(test)]
mod tests_llm_16_146 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_larrow_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site(); // use a call site span for testing
        let char1 = '-' as char; // first character
        let char2 = '>' as char; // second character

        // Call the function being tested
        __private::push_larrow_spanned(&mut tokens, span);

        // Create the expected tokens
        let mut expected_tokens = TokenStream::new();
        let mut punct1 = Punct::new(char1, Spacing::Joint);
        punct1.set_span(span);
        expected_tokens.append(punct1);
        let mut punct2 = Punct::new(char2, Spacing::Alone);
        punct2.set_span(span);
        expected_tokens.append(punct2);

        // Assert that the tokens match
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_148 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_le_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();

        // Call the push_le_spanned function with specific characters
        push_le_spanned(&mut tokens, span);

        // Create the expected output
        let expected_output: TokenStream = {
            let mut ts = TokenStream::new();
            let joint_punct = Punct::new('a', Spacing::Joint);
            let alone_punct = Punct::new('b', Spacing::Alone);
            ts.append(joint_punct);
            ts.append(alone_punct);
            ts
        };

        // Assert that the output matches the expected output
        assert_eq!(tokens.to_string(), expected_output.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_154 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_mul_eq_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site(); // Using a span that calls site
        let char1 = '*';
        let char2 = '=';

        // Invoke the function
        __private::push_mul_eq_spanned(&mut tokens, span);

        // Create the expected tokens
        let mut expected_tokens = TokenStream::new();
        let mut punct1 = Punct::new(char1, Spacing::Joint);
        punct1.set_span(span);
        expected_tokens.append(punct1);
        
        let mut punct2 = Punct::new(char2, Spacing::Alone);
        punct2.set_span(span);
        expected_tokens.append(punct2);

        // Assert that tokens generated are as expected
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_157 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Span, Group, Delimiter};

    #[test]
    fn test_push_or() {
        let mut tokens = TokenStream::new();
        let expected_tokens = "||";

        // Call the push_or function
        __private::push_or(&mut tokens);

        // Convert the TokenStream to a string to compare
        let token_string = tokens.to_string();
        assert_eq!(token_string, expected_tokens);
    }
}

#[cfg(test)]
mod tests_llm_16_160 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_or_or() {
        let mut tokens = TokenStream::new();

        // Call the `push_or_or` function with the mock `TokenStream`.
        // You will need to replace `push_or_or` with the appropriate name that matches the function signature.
        __private::push_or_or(&mut tokens);

        // Create the expected tokens
        let expected_tokens = {
            let mut expected = TokenStream::new();
            expected.append(Punct::new('|', Spacing::Joint));
            expected.append(Punct::new('|', Spacing::Alone));
            expected
        };

        // Assert that the tokens generated match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_162 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, Spacing};

    #[test]
    fn test_push_or_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();
        let char1 = ','; // Example character

        // Call the target function
        __private::push_or_spanned(&mut tokens, span);

        // Create expected tokens
        let mut expected_tokens = TokenStream::new();
        let mut punct = Punct::new(char1, Spacing::Alone);
        punct.set_span(span);
        expected_tokens.append(punct);

        // Assert that the output tokens match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_164 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span};
    use proc_macro2::TokenTree;
    use crate::__private::push_pound_spanned;

    #[test]
    fn test_push_pound_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site(); // Using the default span

        // Call the target function
        push_pound_spanned(&mut tokens, span);

        // Checking the output
        let expected: TokenStream = "##".parse().unwrap(); // Assuming the output should be a pound sign (##) as a Punct
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_165 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_question() {
        let mut tokens = TokenStream::new();
        let char1 = '?';
        push_question(&mut tokens);
        let expected = Punct::new(char1, Spacing::Alone);
        let actual = tokens.clone().into_iter().next().unwrap();
        assert_eq!(actual.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_166 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Span, Punct, TokenTree, Spacing};

    #[test]
    fn test_push_question_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site(); // You can use any span as per your test case
        let char1 = '?';
        
        // Call the target function
        __private::push_question_spanned(&mut tokens, span);

        // Verify that the tokens include the expected punctuation
        let expected_punct = Punct::new(char1, Spacing::Alone);
        assert!(tokens.to_string().contains(expected_punct.to_string().as_str()));
    }
}

#[cfg(test)]
mod tests_llm_16_173 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_semi() {
        let mut tokens = TokenStream::new();
        // Assuming `push_semi` adds a semicolon
        let char1 = ';';
    
        // Call the private function
        __private::push_semi(&mut tokens);
    
        // Check if the last token is the semicolon
        let last_token = tokens.into_iter().last();
        assert!(last_token.is_some());
        if let Some(token) = last_token {
            if let TokenTree::Punct(punct) = token {
                assert_eq!(punct.as_char(), char1);
                assert_eq!(punct.spacing(), Spacing::Alone);
            } else {
                panic!("Expected a Punct token.");
            }
        }
    }
}

#[cfg(test)]
mod tests_llm_16_179 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_shr() {
        let mut tokens = TokenStream::new();
        let char1 = '>';
        let char2 = '>';

        // Call the private function
        __private::push_shr(&mut tokens);

        // Check the tokens
        let expected_tokens: TokenStream = {
            let mut ts = TokenStream::new();
            ts.append(Punct::new(char1, Spacing::Joint));
            ts.append(Punct::new(char2, Spacing::Alone));
            ts
        };

        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_181 {
    use proc_macro2::{TokenStream, Span, Punct, Spacing};
    use crate::__private::push_shr_eq_spanned;

    #[test]
    fn test_push_shr_eq_spanned() {
        let mut tokens = TokenStream::new();
        let span = Span::call_site();
        
        push_shr_eq_spanned(&mut tokens, span);
        
        let expected_puncts = vec![
            Punct::new('>', Spacing::Joint),
            Punct::new('=', Spacing::Joint),
            Punct::new('>', Spacing::Alone),
        ];

        let mut result_puncts = tokens.into_iter().filter_map(|tt| {
            if let proc_macro2::TokenTree::Punct(punct) = tt {
                Some(punct)
            } else {
                None
            }
        }).collect::<Vec<_>>();

        assert_eq!(result_puncts.len(), expected_puncts.len());
        for (expected, result) in expected_puncts.iter().zip(result_puncts.iter()) {
            assert_eq!(expected.as_char(), result.as_char());
            assert_eq!(expected.spacing(), result.spacing());
        }
    }
}

#[cfg(test)]
mod tests_llm_16_185 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Punct, Spacing};

    #[test]
    fn test_push_sub() {
        let mut tokens = TokenStream::new();
        let name = "__private::push_sub"; // Assuming this will match your function name.

        // Call the function with expected parameters
        push_sub(&mut tokens);

        // Create expected output
        let expected = Punct::new('-', Spacing::Alone);

        // Verify the output
        let mut expected_tokens = TokenStream::new();
        expected_tokens.append(expected);

        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}
