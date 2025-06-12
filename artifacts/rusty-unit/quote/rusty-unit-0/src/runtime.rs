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

pub mod get_span {
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
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut isize_0: isize = -12203isize;
    let mut isize_1: isize = 13033isize;
    let mut isize_2: isize = -31796isize;
    let mut isize_3: isize = 648isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "eNVt8IiMu6SEoi";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_4: isize = 8232isize;
    let mut isize_5: isize = 4682isize;
    let mut isize_6: isize = 12872isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut isize_0: isize = -29410isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "unjPIF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = -6899isize;
    let mut isize_2: isize = -13268isize;
    let mut isize_3: isize = 18722isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = -2929isize;
    let mut isize_5: isize = 10430isize;
    let mut isize_6: isize = -12740isize;
    let mut str_1: &str = "eXAjPpCjBMo6za";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_7: isize = -3561isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_7);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspan_7: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut isize_0: isize = 16822isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 4466isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = 3887isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_0: &str = "Wwc1j1Lubuq08NNZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_3: isize = 7936isize;
    let mut str_1: &str = "UyMpU4ZStLipE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_4: isize = -3879isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = 7369isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_6: isize = -2025isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut isize_0: isize = 17263isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 2725isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "Yweus9YeZVnY7dWwM";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = 8558isize;
    let mut str_1: &str = "Vka";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "xrYKPEak";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "5csNw";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "cseUa4cIRNq8n61nIm";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_3: isize = -5982isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_5: &str = "Y";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut isize_4: isize = -804isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut isize_0: isize = 376isize;
    let mut isize_1: isize = 1596isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = -28750isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_3: isize = 12040isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = 11834isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = -7970isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_6: isize = -15712isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut str_0: &str = "h4Dc";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut str_0: &str = "guMcr9Z97i";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = -7790isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "xgjqo8Q97jzNGuAR2";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_1: isize = -3593isize;
    let mut isize_2: isize = -1435isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_3: isize = -3423isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = -8079isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_2: &str = "uSw6og";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "uMLK8gDqDGhx14YIUq";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "67XjGKBbNDmV";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_34() {
    rusty_monitor::set_test_id(34);
    let mut isize_0: isize = 5406isize;
    let mut str_0: &str = "4akfjZFO7zQ8352dhjf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "92vvrwdjx";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_1: isize = -6558isize;
    let mut str_2: &str = "GUNsbqCxy1he4IyOG";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_2: isize = -4715isize;
    let mut str_3: &str = "6sMOI3l";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "3hk";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_3: isize = -9433isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_4: isize = -8438isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut isize_0: isize = -11009isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "N";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 1879isize;
    let mut str_1: &str = "u3Es1jokK8Ow4P";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = -1754isize;
    let mut isize_3: isize = -9192isize;
    let mut isize_4: isize = -6549isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_2: &str = "gz2GhqiHn";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "2x8n";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "9qHGnS";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}
}