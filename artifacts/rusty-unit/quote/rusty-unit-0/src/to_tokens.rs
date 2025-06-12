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
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_9() {
    rusty_monitor::set_test_id(9);
    let mut str_0: &str = "3Ehh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 10436isize;
    let mut isize_1: isize = -4073isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "m1RhFpy5Qy3xDXzILQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "nieF";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "LZww7uxEdbR4IRee";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_2: isize = 17170isize;
    let mut isize_3: isize = 4082isize;
    let mut str_4: &str = "cR6H27d1IemnTuL";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_4: isize = -3116isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut isize_0: isize = 2132isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -7007isize;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = -5149isize;
    let mut isize_3: isize = 11705isize;
    let mut isize_4: isize = -2409isize;
    let mut isize_5: isize = -1733isize;
    let mut isize_6: isize = -8842isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut isize_0: isize = 1524isize;
    let mut str_0: &str = "i0iWAavQIp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 5260isize;
    let mut isize_2: isize = 1339isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_3: isize = -2884isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = 7618isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_5: isize = 4258isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_6: isize = -8889isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut str_0: &str = "eBVrSfh0Mu3kgLHJe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 11371isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "JHZ4LmqVPlf3MeF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_1: isize = -2020isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = -1768isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_2: &str = "OuDwbpI9J9SLebjF";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "H6TGHw";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "guw2n";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_3: isize = -8681isize;
    let mut str_5: &str = "132sjOw6XXSzd8T8";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "BsAEObFnZ6q1Y";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut isize_0: isize = 6607isize;
    let mut isize_1: isize = -10763isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = 6205isize;
    let mut isize_3: isize = -8068isize;
    let mut isize_4: isize = -15421isize;
    let mut isize_5: isize = 19624isize;
    let mut str_0: &str = "POh0zKn523In";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_6: isize = -1085isize;
    let mut str_1: &str = "PRvecUDV6KsscekQWz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_37() {
    rusty_monitor::set_test_id(37);
    let mut isize_0: isize = -1655isize;
    let mut isize_1: isize = 7764isize;
    let mut isize_2: isize = -4267isize;
    let mut isize_3: isize = -1544isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_4: isize = 4866isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "ewrqcLnGpjT12VF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_5: isize = -5976isize;
    let mut str_1: &str = "H06kRPj";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "FCWS3QNQdsvuXBjtZU";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "ba7Jq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_42() {
    rusty_monitor::set_test_id(42);
    let mut str_0: &str = "2kAANTPPFJfzoYyp";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 14150isize;
    let mut isize_1: isize = 12551isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "DbJ4JnjzQOKN8EBPfWc";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = -16237isize;
    let mut isize_3: isize = -9762isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_2: &str = "C6EWnS0DDwjTdCRfq";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_4: isize = -13511isize;
    let mut isize_5: isize = -3886isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut str_0: &str = "vF3M5noSEv1qLjGFDTh";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "qsJX1MQ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = 2676isize;
    let mut isize_1: isize = 5476isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = -11995isize;
    let mut isize_3: isize = 138isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = 498isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_2: &str = "7Cm0VJnYNSkJqvQc1zv";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_5: isize = -8574isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    panic!("From RustyUnit with love");
}
}