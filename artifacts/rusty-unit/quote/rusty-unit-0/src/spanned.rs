use crate::ToTokens;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};

// Not public API other than via the syn crate. Use syn::spanned::Spanned.
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}

impl Spanned for Span {
    fn __span(&self) -> Span {
        *self
    }
}

impl Spanned for DelimSpan {
    fn __span(&self) -> Span {
        self.join()
    }
}

impl<T: ?Sized + ToTokens> Spanned for T {
    fn __span(&self) -> Span {
        join_spans(self.into_token_stream())
    }
}

fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter().map(|tt| tt.span());

    let first = match iter.next() {
        Some(span) => span,
        None => return Span::call_site(),
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.join(last))
        .unwrap_or(first)
}

mod private {
    use crate::ToTokens;
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;

    pub trait Sealed {}
    impl Sealed for Span {}
    impl Sealed for DelimSpan {}
    impl<T: ?Sized + ToTokens> Sealed for T {}
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::ops::Deref;
#[test]
#[should_panic]
fn rusty_test_7() {
    rusty_monitor::set_test_id(7);
    let mut isize_0: isize = -526isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 20483isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = -71isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_3: isize = 21562isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut str_0: &str = "gRBrMxYHL";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_4: isize = -3016isize;
    let mut str_1: &str = "MkKlNCM68O";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_5: isize = 6118isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut isize_6: isize = -3055isize;
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_10() {
    rusty_monitor::set_test_id(10);
    let mut isize_0: isize = 6188isize;
    let mut str_0: &str = "qrFv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "h1KRbDd";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_1: isize = 14532isize;
    let mut isize_2: isize = -8860isize;
    let mut str_2: &str = "4fk2hA";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "Bd4Ocy53ivR";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_3: isize = -2389isize;
    let mut isize_4: isize = -3289isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_5: isize = 9574isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_16() {
    rusty_monitor::set_test_id(16);
    let mut str_0: &str = "FU";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_0: isize = -17356isize;
    let mut isize_1: isize = -3048isize;
    let mut isize_2: isize = -4549isize;
    let mut isize_3: isize = -4864isize;
    let mut str_1: &str = "YPZzFr";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_4: isize = 3810isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_5: isize = 2962isize;
    let mut str_2: &str = "qsPRYj7";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "r";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_18() {
    rusty_monitor::set_test_id(18);
    let mut str_0: &str = "v0n4teQ1Mbre07TVTq";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "m4iF8bN86F6PJUL9o9l";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "oQmbl3K8iGloSYSbHEI";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "o6DHF5JZwUBdNkolqr";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "wK7iBoQk";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "5px";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut isize_0: isize = -4203isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_6: &str = "9AHwX2oJwitWzQ1W1R8";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut isize_1: isize = 5123isize;
    let mut str_7: &str = "wYQLfyN";
    let mut str_7_ref_0: &str = &mut str_7;
    let mut isize_2: isize = -1636isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_8: &str = "K";
    let mut str_8_ref_0: &str = &mut str_8;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut isize_0: isize = 5374isize;
    let mut str_0: &str = "B";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_1: isize = -14893isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_1: &str = "pCTl";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut str_2: &str = "CkVLdZBQuyknl";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "l6";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut str_4: &str = "Ocy22AwrCN0e4m";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut str_5: &str = "mm2tpT";
    let mut str_5_ref_0: &str = &mut str_5;
    let mut isize_2: isize = 1815isize;
    let mut str_6: &str = "49";
    let mut str_6_ref_0: &str = &mut str_6;
    let mut isize_3: isize = -4421isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_25() {
    rusty_monitor::set_test_id(25);
    let mut isize_0: isize = 18132isize;
    let mut isize_1: isize = -583isize;
    let mut isize_2: isize = 2506isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut str_0: &str = "qzNafI7EaPbkyMuczv";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_3: isize = -5603isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut str_1: &str = "";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_4: isize = -6579isize;
    let mut isize_5: isize = -12032isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut isize_6: isize = -9944isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut isize_0: isize = -2170isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_1: isize = 16175isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_2: isize = 16isize;
    let mut str_0: &str = "aL5zpT6fnTGt0qZ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut isize_3: isize = -15676isize;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut str_1: &str = "wS31CjJcg8L98qmd1l";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_4: isize = 15774isize;
    let mut isize_5: isize = 4353isize;
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_5);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut isize_6: isize = 2508isize;
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_6);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_5: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspan_5_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_5;
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_5_ref_0);
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_6: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspan_6_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_6;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_6_ref_0);
    let mut getspaninner_5: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspaninner_6: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
fn rusty_test_47() {
    rusty_monitor::set_test_id(47);
    let mut str_0: &str = "tUdLUYT7ExpLln5HYP";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut str_1: &str = "j";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut isize_0: isize = -7575isize;
    let mut isize_1: isize = -5262isize;
    let mut getspan_0: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_1);
    let mut getspan_0_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_0;
    let mut isize_2: isize = 16100isize;
    let mut str_2: &str = "gfqG88VRIEbFx";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut str_3: &str = "n1wNB";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut isize_3: isize = 1190isize;
    let mut getspan_1: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_3);
    let mut getspan_1_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_1;
    let mut isize_4: isize = -8197isize;
    let mut str_4: &str = "qhNcB4FX";
    let mut str_4_ref_0: &str = &mut str_4;
    let mut getspan_2: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_4);
    let mut getspaninner_0: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_1_ref_0);
    let mut getspan_2_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_2;
    let mut getspaninner_1: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_2_ref_0);
    let mut getspan_3: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_2);
    let mut getspaninner_2: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_0_ref_0);
    let mut getspan_3_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_3;
    let mut getspaninner_3: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_3_ref_0);
    let mut getspan_4: crate::__private::get_span::GetSpan<isize> = crate::__private::get_span(isize_0);
    let mut getspan_4_ref_0: &crate::__private::get_span::GetSpan<isize> = &mut getspan_4;
    let mut getspaninner_4: &crate::__private::get_span::GetSpanInner<isize> = crate::__private::get_span::GetSpan::deref(getspan_4_ref_0);
    panic!("From RustyUnit with love");
}
}