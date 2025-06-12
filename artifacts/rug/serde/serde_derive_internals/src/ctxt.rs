use quote::ToTokens;
use std::cell::RefCell;
use std::fmt::Display;
use std::thread;

/// A type to collect errors together and format them.
///
/// Dropping this object will cause a panic. It must be consumed using `check`.
///
/// References can be shared since this type uses run-time exclusive mut checking.
#[derive(Default)]
pub struct Ctxt {
    // The contents will be set to `None` during checking. This is so that checking can be
    // enforced.
    errors: RefCell<Option<Vec<syn::Error>>>,
}

impl Ctxt {
    /// Create a new context object.
    ///
    /// This object contains no errors, but will still trigger a panic if it is not `check`ed.
    pub fn new() -> Self {
        Ctxt {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    /// Add an error to the context object with a tokenenizable object.
    ///
    /// The object is used for spanning in error messages.
    pub fn error_spanned_by<A: ToTokens, T: Display>(&self, obj: A, msg: T) {
        self.errors
            .borrow_mut()
            .as_mut()
            .unwrap()
            // Curb monomorphization from generating too many identical methods.
            .push(syn::Error::new_spanned(obj.into_token_stream(), msg));
    }

    /// Add one of Syn's parse errors.
    pub fn syn_error(&self, err: syn::Error) {
        self.errors.borrow_mut().as_mut().unwrap().push(err);
    }

    /// Consume this object, producing a formatted error string if there are errors.
    pub fn check(self) -> syn::Result<()> {
        let mut errors = self.errors.borrow_mut().take().unwrap().into_iter();

        let mut combined = match errors.next() {
            Some(first) => first,
            None => return Ok(()),
        };

        for rest in errors {
            combined.combine(rest);
        }

        Err(combined)
    }
}

impl Drop for Ctxt {
    fn drop(&mut self) {
        if !thread::panicking() && self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}

#[cfg(test)]
mod tests_llm_16_2 {
    use super::*;

use crate::*;
    use std::cell::RefCell;
    use std::panic::AssertUnwindSafe;

    #[test]
    #[should_panic(expected = "forgot to check for errors")]
    fn test_drop_panics_when_errors_exist() {
        let ctxt = Ctxt::new();
        ctxt.syn_error(syn::Error::new(proc_macro2::Span::call_site(), "error 1"));
        // ctxt will panic when it goes out of scope without being checked
    }

    #[test]
    fn test_drop_does_not_panic_when_errors_are_checked() {
        let ctxt = Ctxt::new();
        ctxt.syn_error(syn::Error::new(proc_macro2::Span::call_site(), "error 1"));
        let result = ctxt.check();
        assert!(result.is_err()); // Ensure check returns an error.
        // ctxt goes out of scope without panic because errors are checked.
    }

    #[test]
    fn test_drop_does_not_panic_when_no_errors() {
        let ctxt = Ctxt::new();
        // ctxt goes out of scope without panic because there are no errors.
    }
}

#[cfg(test)]
mod tests_llm_16_126 {
    use super::*;

use crate::*;
    use quote::quote;

    #[test]
    fn test_check_no_errors() {
        let ctxt = Ctxt::new();
        assert!(ctxt.check().is_ok());
    }

    #[test]
    fn test_check_with_errors() {
        let ctxt = Ctxt::new();
        ctxt.error_spanned_by(quote! { 42 }, "Test error 1");
        ctxt.error_spanned_by(quote! { 43 }, "Test error 2");
        
        let result = ctxt.check();
        assert!(result.is_err());
        
        if let Err(ref err) = result {
            assert_eq!(err.to_string(), "Test error 1\nTest error 2");
        }
    }

    #[test]
    #[should_panic(expected = "forgot to check for errors")]
    fn test_check_not_called() {
        let ctxt = Ctxt::new();
        ctxt.error_spanned_by(quote! { 42 }, "Test error");
        drop(ctxt);
    }
}

#[cfg(test)]
mod tests_llm_16_128 {
    use super::*;

use crate::*;
    use crate::internals::ctxt::Ctxt;

    #[test]
    fn test_ctxt_new() {
        let ctxt = Ctxt::new();
        assert!(ctxt.errors.borrow().is_some());
        assert!(ctxt.errors.borrow().as_ref().unwrap().is_empty());
    }
}
