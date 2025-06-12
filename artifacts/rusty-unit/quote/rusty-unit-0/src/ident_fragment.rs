use alloc::borrow::Cow;
use core::fmt;
use proc_macro2::{Ident, Span};

/// Specialized formatting trait used by `format_ident!`.
///
/// [`Ident`] arguments formatted using this trait will have their `r#` prefix
/// stripped, if present.
///
/// See [`format_ident!`] for more information.
///
/// [`format_ident!`]: crate::format_ident
pub trait IdentFragment {
    /// Format this value as an identifier fragment.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

    /// Span associated with this `IdentFragment`.
    ///
    /// If non-`None`, may be inherited by formatted identifiers.
    fn span(&self) -> Option<Span> {
        None
    }
}

impl<T: IdentFragment + ?Sized> IdentFragment for &T {
    fn span(&self) -> Option<Span> {
        <T as IdentFragment>::span(*self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(*self, f)
    }
}

impl<T: IdentFragment + ?Sized> IdentFragment for &mut T {
    fn span(&self) -> Option<Span> {
        <T as IdentFragment>::span(*self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(*self, f)
    }
}

impl IdentFragment for Ident {
    fn span(&self) -> Option<Span> {
        Some(self.span())
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = self.to_string();
        if let Some(id) = id.strip_prefix("r#") {
            fmt::Display::fmt(id, f)
        } else {
            fmt::Display::fmt(&id[..], f)
        }
    }
}

impl<T> IdentFragment for Cow<'_, T>
where
    T: IdentFragment + ToOwned + ?Sized,
{
    fn span(&self) -> Option<Span> {
        T::span(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

// Limited set of types which this is implemented for, as we want to avoid types
// which will often include non-identifier characters in their `Display` impl.
macro_rules! ident_fragment_display {
    ($($T:ty),*) => {
        $(
            impl IdentFragment for $T {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt(self, f)
                }
            }
        )*
    };
}

ident_fragment_display!(bool, str, String, char);
ident_fragment_display!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut isize_0: isize = 5928isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "l3I7VFfzj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "iueMZT4BX3nrY43vO6";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_1: isize = -25991isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_2: &str = "";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_2: isize = 8023isize;
    let mut str_3: &str = "3ORuW9rP";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_3: isize = -12511isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = 1187isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = 233isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut isize_0: isize = -3701isize;
    let mut isize_1: isize = 18917isize;
    let mut isize_2: isize = -3702isize;
    let mut isize_3: isize = -9059isize;
    let mut isize_4: isize = -655isize;
    let mut str_0: &str = "wsfwf";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_5: isize = 5582isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_6: isize = -10594isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut isize_0: isize = 2329isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "5UVbOILuKRF9d4kFz";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 15610isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_1: &str = "91a";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = -4833isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = -14159isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_2: &str = "qmQu3hUSazVFI";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "YguioVVhz1O0R2";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_4: isize = -1695isize;
    let mut str_4: &str = "qD42RmF";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "DtRgGb4ZfZJdft";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_21() {
    rusty_monitor::set_test_id(21);
    let mut isize_0: isize = -7622isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "cRGUoPa7mXUq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = -5676isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = 5570isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = 3585isize;
    let mut isize_4: isize = -15042isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_1: &str = "hHV2WmP6S5kUb4V2ZsS";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "hlPvC2kV6Vo8M";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "5KtyezSF34";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "6JGCovpm7dnukK";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "9QvbZZ3nRNFAjtq9Vt1";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut str_0: &str = "OXoG8XycElVgHyXC";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = -6997isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -3038isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = 6967isize;
    let mut isize_3: isize = -8424isize;
    let mut str_1: &str = "DGx5";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "X20EiWwPRZ";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_4: isize = -18598isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_3: &str = "k2pll2uI1408CYNQd";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "8buo";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_29() {
    rusty_monitor::set_test_id(29);
    let mut isize_0: isize = -2393isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 7979isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "NIQwcPfi1SbC55nrJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = 7178isize;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_3: isize = 13817isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = -17044isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = -7692isize;
    let mut str_2: &str = "PexZEvy4Q";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut str_0: &str = "7806BpsNOPXIHQcgj";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 9665isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -8367isize;
    let mut str_1: &str = "MYfLuHVsoyISg9z1bnP";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = -5721isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_2: &str = "hOEPpqhAOZUkhW";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_3: isize = 1324isize;
    let mut isize_4: isize = -16646isize;
    let mut isize_5: isize = 4666isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_3: &str = "O1HlvZ";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_40() {
    rusty_monitor::set_test_id(40);
    let mut isize_0: isize = 1681isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -19061isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = -4431isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_0: &str = "ZQJ9ebci5R9giF";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_3: isize = 20841isize;
    let mut isize_4: isize = 4164isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = -18750isize;
    let mut isize_6: isize = 4115isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
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
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut isize_0: isize = -8404isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "eLz6D";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 727isize;
    let mut isize_2: isize = -2799isize;
    let mut str_1: &str = "TeeiwxAVRc";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_3: isize = 20799isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = -11253isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_5: isize = -640isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_6: isize = 2156isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut isize_0: isize = 10504isize;
    let mut str_0: &str = "gzY";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "TgicBZFRfZbVJcxOE";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "twr1hQpPQAZFdyszzn0";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "YlPs2rGKt5pzuQN";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "HshbGix2KHLk1gZ9eE";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_1: isize = 14491isize;
    let mut str_5: &str = "";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "OGeAuU93xwLc3Zj7";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut isize_2: isize = -1878isize;
    let mut isize_3: isize = 6049isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut isize_0: isize = 1650isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -11975isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_0: &str = "7SfzFfl5N6pTwHH";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = -2380isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = 2320isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_4: isize = 6978isize;
    let mut str_1: &str = "2bbJvgFSyLLSkjRFq6h";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "n0pG";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_5: isize = -444isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}
}