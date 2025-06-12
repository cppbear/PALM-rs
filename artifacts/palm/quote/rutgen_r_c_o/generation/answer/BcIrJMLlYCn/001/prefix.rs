// Answer 0

#[test]
fn test_delim_span_call_site_call_site() {
    let delim_span = proc_macro2::extra::DelimSpan::new(proc_macro2::Span::call_site(), proc_macro2::Span::call_site());
    delim_span.__span();
}

#[test]
fn test_delim_span_def_site_def_site() {
    let delim_span = proc_macro2::extra::DelimSpan::new(proc_macro2::Span::def_site(), proc_macro2::Span::def_site());
    delim_span.__span();
}

#[test]
fn test_delim_span_call_site_def_site() {
    let delim_span = proc_macro2::extra::DelimSpan::new(proc_macro2::Span::call_site(), proc_macro2::Span::def_site());
    delim_span.__span();
}

#[test]
fn test_delim_span_def_site_call_site() {
    let delim_span = proc_macro2::extra::DelimSpan::new(proc_macro2::Span::def_site(), proc_macro2::Span::call_site());
    delim_span.__span();
}

