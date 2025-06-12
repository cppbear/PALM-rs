#[cfg(not(no_serde_derive))]
pub mod de;
#[cfg(not(no_serde_derive))]
pub mod ser;

// FIXME: #[cfg(doctest)] once https://github.com/rust-lang/rust/issues/67295 is fixed.
pub mod doc;

pub use crate::lib::clone::Clone;
pub use crate::lib::convert::{From, Into, TryFrom};
pub use crate::lib::default::Default;
pub use crate::lib::fmt::{self, Formatter};
pub use crate::lib::marker::PhantomData;
pub use crate::lib::option::Option::{self, None, Some};
pub use crate::lib::ptr;
pub use crate::lib::result::Result::{self, Err, Ok};

pub use self::string::from_utf8_lossy;

#[cfg(any(feature = "alloc", feature = "std"))]
pub use crate::lib::{ToString, Vec};

mod string {
    use crate::lib::*;

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }

    // The generated code calls this like:
    //
    //     let value = &_serde::__private::from_utf8_lossy(bytes);
    //     Err(_serde::de::Error::unknown_variant(value, VARIANTS))
    //
    // so it is okay for the return type to be different from the std case as long
    // as the above works.
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    pub fn from_utf8_lossy(bytes: &[u8]) -> &str {
        // Three unicode replacement characters if it fails. They look like a
        // white-on-black question mark. The user will recognize it as invalid
        // UTF-8.
        str::from_utf8(bytes).unwrap_or("\u{fffd}\u{fffd}\u{fffd}")
    }
}

#[cfg(test)]
mod tests_llm_16_2371 {
    use crate::__private::string::from_utf8_lossy;
    use std::borrow::Cow;

    #[test]
    fn test_from_utf8_lossy_valid_utf8() {
        let input = b"Hello, world!";
        let result = from_utf8_lossy(input);
        assert_eq!(result, Cow::Borrowed("Hello, world!"));
    }

    #[test]
    fn test_from_utf8_lossy_invalid_utf8() {
        let input = b"Hello, \xFFworld!";
        let result = from_utf8_lossy(input);
        assert_eq!(result, Cow::Borrowed("Hello, �world!"));
    }

    #[test]
    fn test_from_utf8_lossy_empty() {
        let input = b"";
        let result = from_utf8_lossy(input);
        assert_eq!(result, Cow::Borrowed(""));
    }
}
