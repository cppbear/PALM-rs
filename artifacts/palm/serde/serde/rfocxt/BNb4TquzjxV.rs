use crate::lib::*;
pub use self::impossible::Impossible;
#[cfg(all(not(feature = "std"), no_core_error))]
pub use crate::std_error::Error as StdError;
#[cfg(not(any(feature = "std", no_core_error)))]
pub use core::error::Error as StdError;
#[cfg(feature = "std")]
pub use std::error::Error as StdError;
macro_rules! declare_error_trait {
    (Error : Sized $(+ $($supertrait:ident)::+)*) => {
        #[doc = " Trait used by `Serialize` implementations to generically construct"]
        #[doc = " errors belonging to the `Serializer` against which they are"] #[doc =
        " currently running."] #[doc = ""] #[doc = " # Example implementation"] #[doc =
        ""] #[doc = " The [example data format] presented on the website shows an error"]
        #[doc = " type appropriate for a basic JSON data format."] #[doc = ""] #[doc =
        " [example data format]: https://serde.rs/data-format.html"] pub trait Error :
        Sized $(+ $($supertrait)::+)* { #[doc =
        " Used when a [`Serialize`] implementation encounters any error"] #[doc =
        " while serializing a type."] #[doc = ""] #[doc =
        " The message should not be capitalized and should not end with a"] #[doc =
        " period."] #[doc = ""] #[doc =
        " For example, a filesystem [`Path`] may refuse to serialize"] #[doc =
        " itself if it contains invalid UTF-8 data."] #[doc = ""] #[doc =
        " ```edition2021"] #[doc = " # struct Path;"] #[doc = " #"] #[doc =
        " # impl Path {"] #[doc = " #     fn to_str(&self) -> Option<&str> {"] #[doc =
        " #         unimplemented!()"] #[doc = " #     }"] #[doc = " # }"] #[doc = " #"]
        #[doc = " use serde::ser::{self, Serialize, Serializer};"] #[doc = ""] #[doc =
        " impl Serialize for Path {"] #[doc =
        "     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>"] #[doc =
        "     where"] #[doc = "         S: Serializer,"] #[doc = "     {"] #[doc =
        "         match self.to_str() {"] #[doc =
        "             Some(s) => serializer.serialize_str(s),"] #[doc =
        "             None => Err(ser::Error::custom(\"path contains invalid UTF-8 characters\")),"]
        #[doc = "         }"] #[doc = "     }"] #[doc = " }"] #[doc = " ```"] #[doc = ""]
        #[doc = " [`Path`]: std::path::Path"] #[doc = " [`Serialize`]: crate::Serialize"]
        fn custom < T > (msg : T) -> Self where T : Display; }
    };
}
#[cfg(feature = "std")]
declare_error_trait!(Error : Sized + StdError);
#[cfg(not(feature = "std"))]
declare_error_trait!(Error : Sized + Debug + Display);
fn iterator_len_hint<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    match iter.size_hint() {
        (lo, Some(hi)) if lo == hi => Some(lo),
        _ => None,
    }
}
