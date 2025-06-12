use crate::lib::*;
pub use self::ignored_any::IgnoredAny;
#[cfg(all(not(feature = "std"), no_core_error))]
pub use crate::std_error::Error as StdError;
#[cfg(not(any(feature = "std", no_core_error)))]
pub use core::error::Error as StdError;
#[cfg(feature = "std")]
pub use std::error::Error as StdError;
macro_rules! declare_error_trait {
    (Error : Sized $(+ $($supertrait:ident)::+)*) => {
        #[doc =
        " The `Error` trait allows `Deserialize` implementations to create descriptive"]
        #[doc = " error messages belonging to the `Deserializer` against which they are"]
        #[doc = " currently running."] #[doc = ""] #[doc =
        " Every `Deserializer` declares an `Error` type that encompasses both"] #[doc =
        " general-purpose deserialization errors as well as errors specific to the"]
        #[doc = " particular deserialization format. For example the `Error` type of"]
        #[doc =
        " `serde_json` can represent errors like an invalid JSON escape sequence or an"]
        #[doc =
        " unterminated string literal, in addition to the error cases that are part of"]
        #[doc = " this trait."] #[doc = ""] #[doc =
        " Most deserializers should only need to provide the `Error::custom` method"]
        #[doc = " and inherit the default behavior for the other methods."] #[doc = ""]
        #[doc = " # Example implementation"] #[doc = ""] #[doc =
        " The [example data format] presented on the website shows an error"] #[doc =
        " type appropriate for a basic JSON data format."] #[doc = ""] #[doc =
        " [example data format]: https://serde.rs/data-format.html"] pub trait Error :
        Sized $(+ $($supertrait)::+)* { #[doc =
        " Raised when there is general error when deserializing a type."] #[doc = ""]
        #[doc =
        " The message should not be capitalized and should not end with a period."] #[doc
        = ""] #[doc = " ```edition2021"] #[doc = " # use std::str::FromStr;"] #[doc =
        " #"] #[doc = " # struct IpAddr;"] #[doc = " #"] #[doc =
        " # impl FromStr for IpAddr {"] #[doc = " #     type Err = String;"] #[doc =
        " #"] #[doc = " #     fn from_str(_: &str) -> Result<Self, String> {"] #[doc =
        " #         unimplemented!()"] #[doc = " #     }"] #[doc = " # }"] #[doc = " #"]
        #[doc = " use serde::de::{self, Deserialize, Deserializer};"] #[doc = ""] #[doc =
        " impl<'de> Deserialize<'de> for IpAddr {"] #[doc =
        "     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>"] #[doc =
        "     where"] #[doc = "         D: Deserializer<'de>,"] #[doc = "     {"] #[doc =
        "         let s = String::deserialize(deserializer)?;"] #[doc =
        "         s.parse().map_err(de::Error::custom)"] #[doc = "     }"] #[doc = " }"]
        #[doc = " ```"] fn custom < T > (msg : T) -> Self where T : Display; #[doc =
        " Raised when a `Deserialize` receives a type different from what it was"] #[doc
        = " expecting."] #[doc = ""] #[doc =
        " The `unexp` argument provides information about what type was received."] #[doc
        = " This is the type that was present in the input file or other source data"]
        #[doc = " of the Deserializer."] #[doc = ""] #[doc =
        " The `exp` argument provides information about what type was being"] #[doc =
        " expected. This is the type that is written in the program."] #[doc = ""] #[doc
        = " For example if we try to deserialize a String out of a JSON file"] #[doc =
        " containing an integer, the unexpected type is the integer and the"] #[doc =
        " expected type is the string."] #[cold] fn invalid_type(unexp : Unexpected, exp
        : & dyn Expected) -> Self {
        Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp)) } #[doc
        = " Raised when a `Deserialize` receives a value of the right type but that"]
        #[doc = " is wrong for some other reason."] #[doc = ""] #[doc =
        " The `unexp` argument provides information about what value was received."]
        #[doc = " This is the value that was present in the input file or other source"]
        #[doc = " data of the Deserializer."] #[doc = ""] #[doc =
        " The `exp` argument provides information about what value was being"] #[doc =
        " expected. This is the type that is written in the program."] #[doc = ""] #[doc
        = " For example if we try to deserialize a String out of some binary data"] #[doc
        = " that is not valid UTF-8, the unexpected value is the bytes and the"] #[doc =
        " expected value is a string."] #[cold] fn invalid_value(unexp : Unexpected, exp
        : & dyn Expected) -> Self {
        Error::custom(format_args!("invalid value: {}, expected {}", unexp, exp)) } #[doc
        = " Raised when deserializing a sequence or map and the input data contains"]
        #[doc = " too many or too few elements."] #[doc = ""] #[doc =
        " The `len` argument is the number of elements encountered. The sequence"] #[doc
        = " or map may have expected more arguments or fewer arguments."] #[doc = ""]
        #[doc = " The `exp` argument provides information about what data was being"]
        #[doc = " expected. For example `exp` might say that a tuple of size 6 was"]
        #[doc = " expected."] #[cold] fn invalid_length(len : usize, exp : & dyn
        Expected) -> Self { Error::custom(format_args!("invalid length {}, expected {}",
        len, exp)) } #[doc =
        " Raised when a `Deserialize` enum type received a variant with an"] #[doc =
        " unrecognized name."] #[cold] fn unknown_variant(variant : & str, expected :
        &'static [&'static str]) -> Self { if expected.is_empty() {
        Error::custom(format_args!("unknown variant `{}`, there are no variants",
        variant)) } else {
        Error::custom(format_args!("unknown variant `{}`, expected {}", variant, OneOf {
        names : expected })) } } #[doc =
        " Raised when a `Deserialize` struct type received a field with an"] #[doc =
        " unrecognized name."] #[cold] fn unknown_field(field : & str, expected :
        &'static [&'static str]) -> Self { if expected.is_empty() {
        Error::custom(format_args!("unknown field `{}`, there are no fields", field)) }
        else { Error::custom(format_args!("unknown field `{}`, expected {}", field, OneOf
        { names : expected })) } } #[doc =
        " Raised when a `Deserialize` struct type expected to receive a required"] #[doc
        = " field with a particular name but that field was not present in the"] #[doc =
        " input."] #[cold] fn missing_field(field : &'static str) -> Self {
        Error::custom(format_args!("missing field `{}`", field)) } #[doc =
        " Raised when a `Deserialize` struct type received more than one of the"] #[doc =
        " same field."] #[cold] fn duplicate_field(field : &'static str) -> Self {
        Error::custom(format_args!("duplicate field `{}`", field)) } }
    };
}
#[cfg(feature = "std")]
declare_error_trait!(Error : Sized + StdError);
#[cfg(not(feature = "std"))]
declare_error_trait!(Error : Sized + Debug + Display);
struct WithDecimalPoint(f64);
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Unexpected<'a> {
    /// The input contained a boolean value that was not expected.
    Bool(bool),
    /// The input contained an unsigned integer `u8`, `u16`, `u32` or `u64` that
    /// was not expected.
    Unsigned(u64),
    /// The input contained a signed integer `i8`, `i16`, `i32` or `i64` that
    /// was not expected.
    Signed(i64),
    /// The input contained a floating point `f32` or `f64` that was not
    /// expected.
    Float(f64),
    /// The input contained a `char` that was not expected.
    Char(char),
    /// The input contained a `&str` or `String` that was not expected.
    Str(&'a str),
    /// The input contained a `&[u8]` or `Vec<u8>` that was not expected.
    Bytes(&'a [u8]),
    /// The input contained a unit `()` that was not expected.
    Unit,
    /// The input contained an `Option<T>` that was not expected.
    Option,
    /// The input contained a newtype struct that was not expected.
    NewtypeStruct,
    /// The input contained a sequence that was not expected.
    Seq,
    /// The input contained a map that was not expected.
    Map,
    /// The input contained an enum that was not expected.
    Enum,
    /// The input contained a unit variant that was not expected.
    UnitVariant,
    /// The input contained a newtype variant that was not expected.
    NewtypeVariant,
    /// The input contained a tuple variant that was not expected.
    TupleVariant,
    /// The input contained a struct variant that was not expected.
    StructVariant,
    /// A message stating what uncategorized thing the input contained that was
    /// not expected.
    ///
    /// The message should be a noun or noun phrase, not capitalized and without
    /// a period. An example message is "unoriginal superhero".
    Other(&'a str),
}
impl<'a> fmt::Display for Unexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::Unexpected::*;
        match *self {
            Bool(b) => write!(formatter, "boolean `{}`", b),
            Unsigned(i) => write!(formatter, "integer `{}`", i),
            Signed(i) => write!(formatter, "integer `{}`", i),
            Float(f) => write!(formatter, "floating point `{}`", WithDecimalPoint(f)),
            Char(c) => write!(formatter, "character `{}`", c),
            Str(s) => write!(formatter, "string {:?}", s),
            Bytes(_) => formatter.write_str("byte array"),
            Unit => formatter.write_str("unit value"),
            Option => formatter.write_str("Option value"),
            NewtypeStruct => formatter.write_str("newtype struct"),
            Seq => formatter.write_str("sequence"),
            Map => formatter.write_str("map"),
            Enum => formatter.write_str("enum"),
            UnitVariant => formatter.write_str("unit variant"),
            NewtypeVariant => formatter.write_str("newtype variant"),
            TupleVariant => formatter.write_str("tuple variant"),
            StructVariant => formatter.write_str("struct variant"),
            Other(other) => formatter.write_str(other),
        }
    }
}
