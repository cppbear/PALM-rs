use super::ToTokens;
use core::iter;
use proc_macro2::{TokenStream, TokenTree};

/// TokenStream extension trait with methods for appending tokens.
///
/// This trait is sealed and cannot be implemented outside of the `quote` crate.
pub trait TokenStreamExt: private::Sealed {
    /// For use by `ToTokens` implementations.
    ///
    /// Appends the token specified to this list of tokens.
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>;

    /// For use by `ToTokens` implementations.
    ///
    /// ```
    /// # use quote::{quote, TokenStreamExt, ToTokens};
    /// # use proc_macro2::TokenStream;
    /// #
    /// struct X;
    ///
    /// impl ToTokens for X {
    ///     fn to_tokens(&self, tokens: &mut TokenStream) {
    ///         tokens.append_all(&[true, false]);
    ///     }
    /// }
    ///
    /// let tokens = quote!(#X);
    /// assert_eq!(tokens.to_string(), "true false");
    /// ```
    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all of the items in the iterator `I`, separated by the tokens
    /// `U`.
    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;

    /// For use by `ToTokens` implementations.
    ///
    /// Appends all tokens in the iterator `I`, appending `U` after each
    /// element, including after the last element of the iterator.
    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens;
}

impl TokenStreamExt for TokenStream {
    fn append<U>(&mut self, token: U)
    where
        U: Into<TokenTree>,
    {
        self.extend(iter::once(token.into()));
    }

    fn append_all<I>(&mut self, iter: I)
    where
        I: IntoIterator,
        I::Item: ToTokens,
    {
        for token in iter {
            token.to_tokens(self);
        }
    }

    fn append_separated<I, U>(&mut self, iter: I, op: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {
        for (i, token) in iter.into_iter().enumerate() {
            if i > 0 {
                op.to_tokens(self);
            }
            token.to_tokens(self);
        }
    }

    fn append_terminated<I, U>(&mut self, iter: I, term: U)
    where
        I: IntoIterator,
        I::Item: ToTokens,
        U: ToTokens,
    {
        for token in iter {
            token.to_tokens(self);
            term.to_tokens(self);
        }
    }
}

mod private {
    use proc_macro2::TokenStream;

    pub trait Sealed {}

    impl Sealed for TokenStream {}
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut str_0: &str = "pefbYMFim9sNFzb9HQ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "yGB9u2hE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = 3936isize;
    let mut isize_1: isize = -11388isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_2: &str = "bJ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "AQevVfn9tJhxyLyx";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_2: isize = -676isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_4: &str = "9xrY1t3AAp";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_3: isize = 3378isize;
    let mut isize_4: isize = -7587isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_30() {
    rusty_monitor::set_test_id(30);
    let mut str_0: &str = "nuzcgHNMxjuSALWPW";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 5590isize;
    let mut isize_1: isize = -10410isize;
    let mut isize_2: isize = -7262isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_3: isize = -4168isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_1: &str = "Pq1kKqBz";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_4: isize = 9585isize;
    let mut isize_5: isize = -455isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_2: &str = "L63DCrgo";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_38() {
    rusty_monitor::set_test_id(38);
    let mut isize_0: isize = -424isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 7066isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "jZpDFY9Q";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = 1762isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = -808isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_4: isize = 3803isize;
    let mut isize_5: isize = -14029isize;
    let mut isize_6: isize = 15556isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_39() {
    rusty_monitor::set_test_id(39);
    let mut isize_0: isize = -3623isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -14684isize;
    let mut isize_2: isize = 7335isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "LsXZKqjIjWwpjF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "dz1ZGjWtjn2";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_3: isize = -6022isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = 6690isize;
    let mut isize_5: isize = -6108isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_6: isize = 8367isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}
}