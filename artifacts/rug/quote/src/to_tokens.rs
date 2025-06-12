use super::TokenStreamExt;
use alloc::borrow::Cow;
use alloc::rc::Rc;
use core::iter;
use proc_macro2::{Group, Ident, Literal, Punct, Span, TokenStream, TokenTree};
use std::ffi::{CStr, CString};

/// Types that can be interpolated inside a `quote!` invocation.
pub trait ToTokens {
    /// Write `self` to the given `TokenStream`.
    ///
    /// The token append methods provided by the [`TokenStreamExt`] extension
    /// trait may be useful for implementing `ToTokens`.
    ///
    /// # Example
    ///
    /// Example implementation for a struct representing Rust paths like
    /// `std::cmp::PartialEq`:
    ///
    /// ```
    /// use proc_macro2::{TokenTree, Spacing, Span, Punct, TokenStream};
    /// use quote::{TokenStreamExt, ToTokens};
    ///
    /// pub struct Path {
    ///     pub global: bool,
    ///     pub segments: Vec<PathSegment>,
    /// }
    ///
    /// impl ToTokens for Path {
    ///     fn to_tokens(&self, tokens: &mut TokenStream) {
    ///         for (i, segment) in self.segments.iter().enumerate() {
    ///             if i > 0 || self.global {
    ///                 // Double colon `::`
    ///                 tokens.append(Punct::new(':', Spacing::Joint));
    ///                 tokens.append(Punct::new(':', Spacing::Alone));
    ///             }
    ///             segment.to_tokens(tokens);
    ///         }
    ///     }
    /// }
    /// #
    /// # pub struct PathSegment;
    /// #
    /// # impl ToTokens for PathSegment {
    /// #     fn to_tokens(&self, tokens: &mut TokenStream) {
    /// #         unimplemented!()
    /// #     }
    /// # }
    /// ```
    fn to_tokens(&self, tokens: &mut TokenStream);

    /// Convert `self` directly into a `TokenStream` object.
    ///
    /// This method is implicitly implemented using `to_tokens`, and acts as a
    /// convenience method for consumers of the `ToTokens` trait.
    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    /// Convert `self` directly into a `TokenStream` object.
    ///
    /// This method is implicitly implemented using `to_tokens`, and acts as a
    /// convenience method for consumers of the `ToTokens` trait.
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

impl<T: ?Sized + ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for &mut T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<'a, T: ?Sized + ToOwned + ToTokens> ToTokens for Cow<'a, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ?Sized + ToTokens> ToTokens for Rc<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(t) = self {
            t.to_tokens(tokens);
        }
    }
}

impl ToTokens for str {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::string(self));
    }
}

impl ToTokens for String {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_str().to_tokens(tokens);
    }
}

impl ToTokens for i8 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i8_suffixed(*self));
    }
}

impl ToTokens for i16 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i16_suffixed(*self));
    }
}

impl ToTokens for i32 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i32_suffixed(*self));
    }
}

impl ToTokens for i64 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i64_suffixed(*self));
    }
}

impl ToTokens for i128 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::i128_suffixed(*self));
    }
}

impl ToTokens for isize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::isize_suffixed(*self));
    }
}

impl ToTokens for u8 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u8_suffixed(*self));
    }
}

impl ToTokens for u16 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u16_suffixed(*self));
    }
}

impl ToTokens for u32 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u32_suffixed(*self));
    }
}

impl ToTokens for u64 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u64_suffixed(*self));
    }
}

impl ToTokens for u128 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::u128_suffixed(*self));
    }
}

impl ToTokens for usize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::usize_suffixed(*self));
    }
}

impl ToTokens for f32 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::f32_suffixed(*self));
    }
}

impl ToTokens for f64 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::f64_suffixed(*self));
    }
}

impl ToTokens for char {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::character(*self));
    }
}

impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let word = if *self { "true" } else { "false" };
        tokens.append(Ident::new(word, Span::call_site()));
    }
}

impl ToTokens for CStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::c_string(self));
    }
}

impl ToTokens for CString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Literal::c_string(self));
    }
}

impl ToTokens for Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

impl ToTokens for Punct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

impl ToTokens for TokenTree {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.clone());
    }
}

impl ToTokens for TokenStream {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(iter::once(self.clone()));
    }

    fn into_token_stream(self) -> TokenStream {
        self
    }
}

#[cfg(test)]
mod tests_llm_16_7 {
    use super::*;

use crate::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_to_tokens() {
        let ts1 = TokenStream::new();
        let mut ts2 = TokenStream::new();
        
        ts1.to_tokens(&mut ts2);
        
        assert_eq!(ts1.to_string(), ts2.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_32 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens_character() {
        let character: char = 'a';
        let mut tokens = TokenStream::new();
        character.to_tokens(&mut tokens);

        let expected = Literal::character(character).to_string();
        assert!(tokens.to_string().contains(&expected));
    }
}

#[cfg(test)]
mod tests_llm_16_34 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens() {
        let value: f64 = 3.14;
        let mut tokens = TokenStream::new();
        
        value.to_tokens(&mut tokens);
        
        let expected = Literal::f64_suffixed(value).to_string();
        let actual = tokens.to_string();

        assert_eq!(actual, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_35 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens() {
        let value: i128 = 12345678901234567890;
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);
        
        let expected_tokens = Literal::i128_suffixed(value).to_string();
        assert!(tokens.to_string().contains(&expected_tokens));
    }
}

#[cfg(test)]
mod tests_llm_16_37 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens() {
        let value: i32 = 42;
        let mut tokens = TokenStream::new();
        
        // Call the to_tokens method
        value.to_tokens(&mut tokens);

        // Create expected tokens
        let expected_tokens = quote! { 42i32 };

        // Assert that the output tokens match the expected tokens
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_38 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens() {
        let value: i64 = 42;
        let mut tokens = TokenStream::new();

        value.to_tokens(&mut tokens);

        let expected = Literal::i64_suffixed(value).to_string();
        let actual = tokens.to_string();
        
        assert!(actual.contains(&expected));
    }
}

#[cfg(test)]
mod tests_llm_16_39 {
    use super::*; // Assuming the relevant traits are in the same module

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens() {
        let value: i8 = 42;
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);

        let expected = Literal::i8_suffixed(42).to_string();
        let actual = tokens.to_string();

        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod tests_llm_16_53 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, TokenTree};

    #[test]
    fn test_to_tokens() {
        let original_tokens: TokenStream = TokenStream::from(TokenTree::Ident(Ident::new("test", Span::call_site())));
        let mut output_tokens = TokenStream::new();

        original_tokens.to_tokens(&mut output_tokens);

        assert_eq!(output_tokens.to_string(), original_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_54 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, TokenTree};

    #[test]
    fn test_to_tokens() {
        let mut tokens = TokenStream::new();
        let token = TokenTree::Ident(proc_macro2::Ident::new("test", proc_macro2::Span::call_site()));

        token.to_tokens(&mut tokens);

        let expected_tokens: TokenStream = TokenTree::Ident(proc_macro2::Ident::new("test", proc_macro2::Span::call_site())).into();
        assert_eq!(tokens.to_string(), expected_tokens.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_61 {
    use super::*;

use crate::*;
    use std::ffi::CStr;
    use proc_macro2::TokenStream;
    use crate::ToTokens;

    #[test]
    fn test_cstr_to_tokens() {
        let cstr = CStr::from_bytes_with_nul(b"test\0").unwrap();
        let mut tokens = TokenStream::new();
        cstr.to_tokens(&mut tokens);

        // Expected outcome can vary based on how `Literal::c_string` is implemented.
        // Here we check if tokens have been appended.
        assert!(!tokens.is_empty());
    }
}

#[cfg(test)]
mod tests_llm_16_62 {
    use super::*;

use crate::*;
    use std::ffi::CString;
    use proc_macro2::TokenStream;
    use crate::ToTokens;
    
    #[test]
    fn test_cstring_to_tokens() {
        let test_str = CString::new("Hello, world!").unwrap();
        let mut tokens = TokenStream::new();
        test_str.to_tokens(&mut tokens);
        
        // You can check if tokens are generated as expected
        // Since `Literal::c_string(self)` creates a `TokenStream`, you may want to check
        // if the tokens contain the expected representation for the CString
        // Placeholder for expected assertion
        // For example, you could use tokens.to_string() to verify the output
        // assert_eq!(tokens.to_string(), "expected_output_here");
    }
}

#[cfg(test)]
mod tests_llm_16_71 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};
    
    #[test]
    fn test_to_tokens() {
        let value: u128 = 42;
        let mut tokens = TokenStream::new();
        
        value.to_tokens(&mut tokens);
        
        let expected = Literal::u128_suffixed(value).to_string();
        let actual = tokens.to_string();
        
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod tests_llm_16_73 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens_u16() {
        let value: u16 = 42;
        let mut tokens = TokenStream::new();
        
        value.to_tokens(&mut tokens);

        let expected_token = Literal::u16_suffixed(42).to_token_stream();
        assert_eq!(tokens.to_string(), expected_token.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_75 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_u32_to_tokens() {
        let value: u32 = 42;
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);

        let expected = Literal::u32_suffixed(42).to_string();
        assert_eq!(tokens.to_string(), expected);
    }
}

#[cfg(test)]
mod tests_llm_16_77 {
    use proc_macro2::{TokenStream, Literal};
    use crate::ToTokens;

    #[test]
    fn test_u64_to_tokens() {
        let value: u64 = 42;
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);

        let expected = Literal::u64_suffixed(42).to_string();
        assert!(tokens.to_string().contains(&expected));
    }
}

#[cfg(test)]
mod tests_llm_16_81 {
    use super::*;

use crate::*;
    use proc_macro2::{TokenStream, Literal};

    #[test]
    fn test_to_tokens_usize() {
        let value: usize = 42;
        let mut tokens = TokenStream::new();
        value.to_tokens(&mut tokens);

        let expected = Literal::usize_suffixed(42).to_token_stream();
        assert_eq!(tokens.to_string(), expected.to_string());
    }
}

#[cfg(test)]
mod tests_llm_16_194 {
    use super::*;

use crate::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_into_token_stream() {
        let initial: TokenStream = TokenStream::new();
        let result = initial.clone().into_token_stream();
        assert_eq!(result.to_string(), initial.to_string());
    }
}
