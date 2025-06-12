/// Formatting macro for constructing `Ident`s.
///
/// <br>
///
/// # Syntax
///
/// Syntax is copied from the [`format!`] macro, supporting both positional and
/// named arguments.
///
/// Only a limited set of formatting traits are supported. The current mapping
/// of format types to traits is:
///
/// * `{}` ⇒ [`IdentFragment`]
/// * `{:o}` ⇒ [`Octal`](std::fmt::Octal)
/// * `{:x}` ⇒ [`LowerHex`](std::fmt::LowerHex)
/// * `{:X}` ⇒ [`UpperHex`](std::fmt::UpperHex)
/// * `{:b}` ⇒ [`Binary`](std::fmt::Binary)
///
/// See [`std::fmt`] for more information.
///
/// <br>
///
/// # IdentFragment
///
/// Unlike `format!`, this macro uses the [`IdentFragment`] formatting trait by
/// default. This trait is like `Display`, with a few differences:
///
/// * `IdentFragment` is only implemented for a limited set of types, such as
///   unsigned integers and strings.
/// * [`Ident`] arguments will have their `r#` prefixes stripped, if present.
///
/// [`IdentFragment`]: crate::IdentFragment
/// [`Ident`]: proc_macro2::Ident
///
/// <br>
///
/// # Hygiene
///
/// The [`Span`] of the first `Ident` argument is used as the span of the final
/// identifier, falling back to [`Span::call_site`] when no identifiers are
/// provided.
///
/// ```
/// # use quote::format_ident;
/// # let ident = format_ident!("Ident");
/// // If `ident` is an Ident, the span of `my_ident` will be inherited from it.
/// let my_ident = format_ident!("My{}{}", ident, "IsCool");
/// assert_eq!(my_ident, "MyIdentIsCool");
/// ```
///
/// Alternatively, the span can be overridden by passing the `span` named
/// argument.
///
/// ```
/// # use quote::format_ident;
/// # const IGNORE_TOKENS: &'static str = stringify! {
/// let my_span = /* ... */;
/// # };
/// # let my_span = proc_macro2::Span::call_site();
/// format_ident!("MyIdent", span = my_span);
/// ```
///
/// [`Span`]: proc_macro2::Span
/// [`Span::call_site`]: proc_macro2::Span::call_site
///
/// <p><br></p>
///
/// # Panics
///
/// This method will panic if the resulting formatted string is not a valid
/// identifier.
///
/// <br>
///
/// # Examples
///
/// Composing raw and non-raw identifiers:
/// ```
/// # use quote::format_ident;
/// let my_ident = format_ident!("My{}", "Ident");
/// assert_eq!(my_ident, "MyIdent");
///
/// let raw = format_ident!("r#Raw");
/// assert_eq!(raw, "r#Raw");
///
/// let my_ident_raw = format_ident!("{}Is{}", my_ident, raw);
/// assert_eq!(my_ident_raw, "MyIdentIsRaw");
/// ```
///
/// Integer formatting options:
/// ```
/// # use quote::format_ident;
/// let num: u32 = 10;
///
/// let decimal = format_ident!("Id_{}", num);
/// assert_eq!(decimal, "Id_10");
///
/// let octal = format_ident!("Id_{:o}", num);
/// assert_eq!(octal, "Id_12");
///
/// let binary = format_ident!("Id_{:b}", num);
/// assert_eq!(binary, "Id_1010");
///
/// let lower_hex = format_ident!("Id_{:x}", num);
/// assert_eq!(lower_hex, "Id_a");
///
/// let upper_hex = format_ident!("Id_{:X}", num);
/// assert_eq!(upper_hex, "Id_A");
/// ```
#[macro_export]
macro_rules! format_ident {
    ($fmt:expr) => {
        $crate::format_ident_impl!([
            $crate::__private::Option::None,
            $fmt
        ])
    };

    ($fmt:expr, $($rest:tt)*) => {
        $crate::format_ident_impl!([
            $crate::__private::Option::None,
            $fmt
        ] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! format_ident_impl {
    // Final state
    ([$span:expr, $($fmt:tt)*]) => {
        $crate::__private::mk_ident(
            &$crate::__private::format!($($fmt)*),
            $span,
        )
    };

    // Span argument
    ([$old:expr, $($fmt:tt)*] span = $span:expr) => {
        $crate::format_ident_impl!([$old, $($fmt)*] span = $span,)
    };
    ([$old:expr, $($fmt:tt)*] span = $span:expr, $($rest:tt)*) => {
        $crate::format_ident_impl!([
            $crate::__private::Option::Some::<$crate::__private::Span>($span),
            $($fmt)*
        ] $($rest)*)
    };

    // Named argument
    ([$span:expr, $($fmt:tt)*] $name:ident = $arg:expr) => {
        $crate::format_ident_impl!([$span, $($fmt)*] $name = $arg,)
    };
    ([$span:expr, $($fmt:tt)*] $name:ident = $arg:expr, $($rest:tt)*) => {
        match $crate::__private::IdentFragmentAdapter(&$arg) {
            arg => $crate::format_ident_impl!([$span.or(arg.span()), $($fmt)*, $name = arg] $($rest)*),
        }
    };

    // Positional argument
    ([$span:expr, $($fmt:tt)*] $arg:expr) => {
        $crate::format_ident_impl!([$span, $($fmt)*] $arg,)
    };
    ([$span:expr, $($fmt:tt)*] $arg:expr, $($rest:tt)*) => {
        match $crate::__private::IdentFragmentAdapter(&$arg) {
            arg => $crate::format_ident_impl!([$span.or(arg.span()), $($fmt)*, arg] $($rest)*),
        }
    };
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut isize_0: isize = -621isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "Z";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = -22772isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_1: &str = "Jawbc5K";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = 4509isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_2: &str = "ezOlV4wuB";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "L8EsbG";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "W7DUMuXplUfMxoTX6";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut isize_3: isize = 16856isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_5: &str = "PCxDEr1n8ZP";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut str_6: &str = "4wxgtQlWyqS";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut str_0: &str = "OTh363K53ki";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = -146isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -3181isize;
    let mut isize_2: isize = 18489isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_3: isize = 5093isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = -8669isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_5: isize = 3287isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_6: isize = 4376isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut isize_0: isize = -8354isize;
    let mut isize_1: isize = 21978isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = -872isize;
    let mut isize_3: isize = 3981isize;
    let mut isize_4: isize = 101isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_5: isize = -19227isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_0: &str = "BrEWMQBJtHUwyCu";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "D";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_6: isize = -1034isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_11() {
    rusty_monitor::set_test_id(11);
    let mut isize_0: isize = 2183isize;
    let mut isize_1: isize = 4797isize;
    let mut str_0: &str = "neoVDDxD";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_2: isize = 2629isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_3: isize = -1458isize;
    let mut str_1: &str = "oTwB23vIkHiWnqLofZ";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "XsbJv1qN4SHY";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_4: isize = -12040isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_5: isize = -16050isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut isize_0: isize = -6105isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "yBuGVt7uX";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 1956isize;
    let mut isize_2: isize = -16616isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_3: isize = 5809isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_4: isize = -16273isize;
    let mut isize_5: isize = 22339isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_6: isize = 3667isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
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

#[test]
#[should_panic]
fn rusty_test_17() {
    rusty_monitor::set_test_id(17);
    let mut isize_0: isize = 3221isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -338isize;
    let mut isize_2: isize = 1733isize;
    let mut isize_3: isize = 10256isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = 16684isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_5: isize = -2558isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_0: &str = "u9tZJx3VHiwkjy2W5h0";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_6: isize = -17047isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_7: isize = 27534isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_7);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_7: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_24() {
    rusty_monitor::set_test_id(24);
    let mut str_0: &str = "oNORwWXLWGZ6bl96";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = 1829isize;
    let mut isize_1: isize = 11409isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "kpp";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "48ROJpcIkut";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_2: isize = 8964isize;
    let mut isize_3: isize = 14503isize;
    let mut isize_4: isize = -4104isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_3: &str = "TsY";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "bzVZ0";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
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
fn rusty_test_33() {
    rusty_monitor::set_test_id(33);
    let mut isize_0: isize = 3210isize;
    let mut str_0: &str = "Vj5dcs";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = 16918isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = -2244isize;
    let mut str_1: &str = "qNFrlQntUVOgC";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "G2OHZnVD7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut isize_3: isize = -3999isize;
    let mut isize_4: isize = 3962isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_5: isize = 17812isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_3: &str = "O1TqEg";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_35() {
    rusty_monitor::set_test_id(35);
    let mut isize_0: isize = -6725isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -10261isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = 1875isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = 1019isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_4: isize = 15913isize;
    let mut str_0: &str = "H3Hv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_5: isize = 8758isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_6: isize = -18915isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut isize_0: isize = -1085isize;
    let mut isize_1: isize = 12089isize;
    let mut str_0: &str = "U";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "WA";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_2: isize = 17019isize;
    let mut isize_3: isize = -10778isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_4: isize = -9689isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_5: isize = 4553isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_6: isize = 2131isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut isize_0: isize = 324isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = -8095isize;
    let mut isize_2: isize = -1317isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_3: isize = 8781isize;
    let mut isize_4: isize = 19653isize;
    let mut isize_5: isize = -19642isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_0: &str = "k";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_6: isize = -7072isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_7: isize = 301isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_7);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspan_7: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}
}