use crate::error::{Error, ErrorCode, Result};
#[cfg(feature = "float_roundtrip")]
use crate::lexical;
use crate::number::Number;
use crate::read::{self, Fused, Reference};
use alloc::string::String;
use alloc::vec::Vec;
#[cfg(feature = "float_roundtrip")]
use core::iter;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::result;
use core::str::FromStr;
use serde::de::{self, Expected, Unexpected};
use serde::forward_to_deserialize_any;
#[cfg(feature = "arbitrary_precision")]
use crate::number::NumberDeserializer;
pub use crate::read::{Read, SliceRead, StrRead};
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub use crate::read::IoRead;
#[cfg(not(feature = "float_roundtrip"))]
static POW10: [f64; 309] = [
    1e000,
    1e001,
    1e002,
    1e003,
    1e004,
    1e005,
    1e006,
    1e007,
    1e008,
    1e009,
    1e010,
    1e011,
    1e012,
    1e013,
    1e014,
    1e015,
    1e016,
    1e017,
    1e018,
    1e019,
    1e020,
    1e021,
    1e022,
    1e023,
    1e024,
    1e025,
    1e026,
    1e027,
    1e028,
    1e029,
    1e030,
    1e031,
    1e032,
    1e033,
    1e034,
    1e035,
    1e036,
    1e037,
    1e038,
    1e039,
    1e040,
    1e041,
    1e042,
    1e043,
    1e044,
    1e045,
    1e046,
    1e047,
    1e048,
    1e049,
    1e050,
    1e051,
    1e052,
    1e053,
    1e054,
    1e055,
    1e056,
    1e057,
    1e058,
    1e059,
    1e060,
    1e061,
    1e062,
    1e063,
    1e064,
    1e065,
    1e066,
    1e067,
    1e068,
    1e069,
    1e070,
    1e071,
    1e072,
    1e073,
    1e074,
    1e075,
    1e076,
    1e077,
    1e078,
    1e079,
    1e080,
    1e081,
    1e082,
    1e083,
    1e084,
    1e085,
    1e086,
    1e087,
    1e088,
    1e089,
    1e090,
    1e091,
    1e092,
    1e093,
    1e094,
    1e095,
    1e096,
    1e097,
    1e098,
    1e099,
    1e100,
    1e101,
    1e102,
    1e103,
    1e104,
    1e105,
    1e106,
    1e107,
    1e108,
    1e109,
    1e110,
    1e111,
    1e112,
    1e113,
    1e114,
    1e115,
    1e116,
    1e117,
    1e118,
    1e119,
    1e120,
    1e121,
    1e122,
    1e123,
    1e124,
    1e125,
    1e126,
    1e127,
    1e128,
    1e129,
    1e130,
    1e131,
    1e132,
    1e133,
    1e134,
    1e135,
    1e136,
    1e137,
    1e138,
    1e139,
    1e140,
    1e141,
    1e142,
    1e143,
    1e144,
    1e145,
    1e146,
    1e147,
    1e148,
    1e149,
    1e150,
    1e151,
    1e152,
    1e153,
    1e154,
    1e155,
    1e156,
    1e157,
    1e158,
    1e159,
    1e160,
    1e161,
    1e162,
    1e163,
    1e164,
    1e165,
    1e166,
    1e167,
    1e168,
    1e169,
    1e170,
    1e171,
    1e172,
    1e173,
    1e174,
    1e175,
    1e176,
    1e177,
    1e178,
    1e179,
    1e180,
    1e181,
    1e182,
    1e183,
    1e184,
    1e185,
    1e186,
    1e187,
    1e188,
    1e189,
    1e190,
    1e191,
    1e192,
    1e193,
    1e194,
    1e195,
    1e196,
    1e197,
    1e198,
    1e199,
    1e200,
    1e201,
    1e202,
    1e203,
    1e204,
    1e205,
    1e206,
    1e207,
    1e208,
    1e209,
    1e210,
    1e211,
    1e212,
    1e213,
    1e214,
    1e215,
    1e216,
    1e217,
    1e218,
    1e219,
    1e220,
    1e221,
    1e222,
    1e223,
    1e224,
    1e225,
    1e226,
    1e227,
    1e228,
    1e229,
    1e230,
    1e231,
    1e232,
    1e233,
    1e234,
    1e235,
    1e236,
    1e237,
    1e238,
    1e239,
    1e240,
    1e241,
    1e242,
    1e243,
    1e244,
    1e245,
    1e246,
    1e247,
    1e248,
    1e249,
    1e250,
    1e251,
    1e252,
    1e253,
    1e254,
    1e255,
    1e256,
    1e257,
    1e258,
    1e259,
    1e260,
    1e261,
    1e262,
    1e263,
    1e264,
    1e265,
    1e266,
    1e267,
    1e268,
    1e269,
    1e270,
    1e271,
    1e272,
    1e273,
    1e274,
    1e275,
    1e276,
    1e277,
    1e278,
    1e279,
    1e280,
    1e281,
    1e282,
    1e283,
    1e284,
    1e285,
    1e286,
    1e287,
    1e288,
    1e289,
    1e290,
    1e291,
    1e292,
    1e293,
    1e294,
    1e295,
    1e296,
    1e297,
    1e298,
    1e299,
    1e300,
    1e301,
    1e302,
    1e303,
    1e304,
    1e305,
    1e306,
    1e307,
    1e308,
];
macro_rules! overflow {
    ($a:ident * 10 + $b:ident, $c:expr) => {
        match $c { c => $a >= c / 10 && ($a > c / 10 || $b > c % 10), }
    };
}
macro_rules! deserialize_number {
    ($method:ident) => {
        deserialize_number!($method, deserialize_number);
    };
    ($method:ident, $using:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value > where V : de::Visitor
        <'de >, { self.$using (visitor) }
    };
}
#[cfg(not(feature = "unbounded_depth"))]
macro_rules! if_checking_recursion_limit {
    ($($body:tt)*) => {
        $($body)*
    };
}
#[cfg(feature = "unbounded_depth")]
macro_rules! if_checking_recursion_limit {
    ($this:ident $($body:tt)*) => {
        if !$this .disable_recursion_limit { $this $($body)* }
    };
}
macro_rules! check_recursion {
    ($this:ident $($body:tt)*) => {
        if_checking_recursion_limit! { $this .remaining_depth -= 1; if $this
        .remaining_depth == 0 { return Err($this
        .peek_error(ErrorCode::RecursionLimitExceeded)); } } $this $($body)*
        if_checking_recursion_limit! { $this .remaining_depth += 1; }
    };
}
macro_rules! deserialize_numeric_key {
    ($method:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value > where V : de::Visitor
        <'de >, { self.deserialize_number(visitor) }
    };
    ($method:ident, $delegate:ident) => {
        fn $method < V > (self, visitor : V) -> Result < V::Value > where V : de::Visitor
        <'de >, { self.de.eat_char(); match tri!(self.de.peek()) { Some(b'0'..= b'9' |
        b'-') => {} _ => return Err(self.de.error(ErrorCode::ExpectedNumericKey)), } let
        value = tri!(self.de.$delegate (visitor)); match tri!(self.de.peek()) {
        Some(b'"') => self.de.eat_char(), _ => return Err(self.de
        .peek_error(ErrorCode::ExpectedDoubleQuote)), } Ok(value) }
    };
}
pub trait Read<'de>: private::Sealed {
    const should_early_return_if_failed: bool;
    fn next(&mut self) -> Result<Option<u8>>;
    fn peek(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);
    fn position(&self) -> Position;
    fn peek_position(&self) -> Position;
    fn byte_offset(&self) -> usize;
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>>;
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;
    fn ignore_str(&mut self) -> Result<()>;
    fn decode_hex_escape(&mut self) -> Result<u16>;
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self);
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;
    fn set_failed(&mut self, failed: &mut bool);
}
pub struct Deserializer<R> {
    read: R,
    scratch: Vec<u8>,
    remaining_depth: u8,
    #[cfg(feature = "float_roundtrip")]
    single_precision: bool,
    #[cfg(feature = "unbounded_depth")]
    disable_recursion_limit: bool,
}
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),
    /// Some I/O error occurred while serializing or deserializing.
    Io(io::Error),
    /// EOF while parsing a list.
    EofWhileParsingList,
    /// EOF while parsing an object.
    EofWhileParsingObject,
    /// EOF while parsing a string.
    EofWhileParsingString,
    /// EOF while parsing a JSON value.
    EofWhileParsingValue,
    /// Expected this character to be a `':'`.
    ExpectedColon,
    /// Expected this character to be either a `','` or a `']'`.
    ExpectedListCommaOrEnd,
    /// Expected this character to be either a `','` or a `'}'`.
    ExpectedObjectCommaOrEnd,
    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,
    /// Expected this character to start a JSON value.
    ExpectedSomeValue,
    /// Expected this character to be a `"`.
    ExpectedDoubleQuote,
    /// Invalid hex escape code.
    InvalidEscape,
    /// Invalid number.
    InvalidNumber,
    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,
    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,
    /// Control character found while parsing a string.
    ControlCharacterWhileParsingString,
    /// Object key is not a string.
    KeyMustBeAString,
    /// Contents of key were supposed to be a number.
    ExpectedNumericKey,
    /// Object key is a non-finite float value.
    FloatKeyMustBeFinite,
    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,
    /// JSON has a comma after the last value in an array or map.
    TrailingComma,
    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,
    /// Unexpected end of hex escape.
    UnexpectedEndOfHexEscape,
    /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    RecursionLimitExceeded,
}
impl<'de, R: Read<'de>> Deserializer<R> {
    pub fn end(&mut self) -> Result<()> {}
    pub fn into_iter<T>(self) -> StreamDeserializer<'de, R, T>
    where
        T: de::Deserialize<'de>,
    {}
    #[cfg(feature = "unbounded_depth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "unbounded_depth")))]
    pub fn disable_recursion_limit(&mut self) {}
    pub(crate) fn peek(&mut self) -> Result<Option<u8>> {}
    fn peek_or_null(&mut self) -> Result<u8> {
        Ok(tri!(self.peek()).unwrap_or(b'\x00'))
    }
    fn eat_char(&mut self) {
        self.read.discard();
    }
    fn next_char(&mut self) -> Result<Option<u8>> {
        self.read.next()
    }
    fn next_char_or_null(&mut self) -> Result<u8> {}
    #[cold]
    fn error(&self, reason: ErrorCode) -> Error {
        let position = self.read.position();
        Error::syntax(reason, position.line, position.column)
    }
    #[cold]
    fn peek_error(&self, reason: ErrorCode) -> Error {}
    fn parse_whitespace(&mut self) -> Result<Option<u8>> {}
    #[cold]
    fn peek_invalid_type(&mut self, exp: &dyn Expected) -> Error {}
    pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {}
    #[cfg(feature = "float_roundtrip")]
    pub(crate) fn do_deserialize_f32<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {}
    pub(crate) fn do_deserialize_i128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {}
    pub(crate) fn do_deserialize_u128<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {}
    fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {}
    #[cold]
    fn fix_position(&self, err: Error) -> Error {}
    fn parse_ident(&mut self, ident: &[u8]) -> Result<()> {}
    fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber> {}
    fn parse_number(
        &mut self,
        positive: bool,
        significand: u64,
    ) -> Result<ParserNumber> {}
    fn parse_decimal(
        &mut self,
        positive: bool,
        mut significand: u64,
        exponent_before_decimal_point: i32,
    ) -> Result<f64> {}
    fn parse_exponent(
        &mut self,
        positive: bool,
        significand: u64,
        starting_exp: i32,
    ) -> Result<f64> {
        self.eat_char();
        let positive_exp = match tri!(self.peek_or_null()) {
            b'+' => {
                self.eat_char();
                true
            }
            b'-' => {
                self.eat_char();
                false
            }
            _ => true,
        };
        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };
        let mut exp = match next {
            c @ b'0'..=b'9' => (c - b'0') as i32,
            _ => {
                return Err(self.error(ErrorCode::InvalidNumber));
            }
        };
        while let c @ b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
            let digit = (c - b'0') as i32;
            if overflow!(exp * 10 + digit, i32::MAX) {
                let zero_significand = significand == 0;
                return self
                    .parse_exponent_overflow(positive, zero_significand, positive_exp);
            }
            exp = exp * 10 + digit;
        }
        let final_exp = if positive_exp {
            starting_exp.saturating_add(exp)
        } else {
            starting_exp.saturating_sub(exp)
        };
        self.f64_from_parts(positive, significand, final_exp)
    }
    #[cfg(not(feature = "float_roundtrip"))]
    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        mut exponent: i32,
    ) -> Result<f64> {
        let mut f = significand as f64;
        loop {
            match POW10.get(exponent.wrapping_abs() as usize) {
                Some(&pow) => {
                    if exponent >= 0 {
                        f *= pow;
                        if f.is_infinite() {
                            return Err(self.error(ErrorCode::NumberOutOfRange));
                        }
                    } else {
                        f /= pow;
                    }
                    break;
                }
                None => {
                    if f == 0.0 {
                        break;
                    }
                    if exponent >= 0 {
                        return Err(self.error(ErrorCode::NumberOutOfRange));
                    }
                    f /= 1e308;
                    exponent += 308;
                }
            }
        }
        Ok(if positive { f } else { -f })
    }
    #[cfg(not(feature = "float_roundtrip"))]
    fn f64_from_parts(
        &mut self,
        positive: bool,
        significand: u64,
        mut exponent: i32,
    ) -> Result<f64> {}
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    #[inline(never)]
    fn parse_long_integer(
        &mut self,
        positive: bool,
        partial_significand: u64,
    ) -> Result<f64> {}
    #[cfg(not(feature = "float_roundtrip"))]
    #[cold]
    #[inline(never)]
    fn parse_long_integer(&mut self, positive: bool, significand: u64) -> Result<f64> {}
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    fn parse_long_decimal(&mut self, positive: bool, integer_end: usize) -> Result<f64> {}
    #[cfg(feature = "float_roundtrip")]
    fn parse_long_exponent(
        &mut self,
        positive: bool,
        integer_end: usize,
    ) -> Result<f64> {}
    #[cfg(feature = "float_roundtrip")]
    #[cold]
    #[inline(never)]
    fn parse_decimal_overflow(
        &mut self,
        positive: bool,
        significand: u64,
        exponent: i32,
    ) -> Result<f64> {}
    #[cfg(not(feature = "float_roundtrip"))]
    #[cold]
    #[inline(never)]
    fn parse_decimal_overflow(
        &mut self,
        positive: bool,
        significand: u64,
        exponent: i32,
    ) -> Result<f64> {}
    #[cold]
    #[inline(never)]
    fn parse_exponent_overflow(
        &mut self,
        positive: bool,
        zero_significand: bool,
        positive_exp: bool,
    ) -> Result<f64> {
        if !zero_significand && positive_exp {
            return Err(self.error(ErrorCode::NumberOutOfRange));
        }
        while let b'0'..=b'9' = tri!(self.peek_or_null()) {
            self.eat_char();
        }
        Ok(if positive { 0.0 } else { -0.0 })
    }
    #[cfg(feature = "float_roundtrip")]
    fn f64_long_from_parts(
        &mut self,
        positive: bool,
        integer_end: usize,
        exponent: i32,
    ) -> Result<f64> {}
    fn parse_any_signed_number(&mut self) -> Result<ParserNumber> {}
    #[cfg(not(feature = "arbitrary_precision"))]
    fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber> {}
    #[cfg(feature = "arbitrary_precision")]
    fn parse_any_number(&mut self, positive: bool) -> Result<ParserNumber> {}
    #[cfg(feature = "arbitrary_precision")]
    fn scan_or_eof(&mut self, buf: &mut String) -> Result<u8> {}
    #[cfg(feature = "arbitrary_precision")]
    fn scan_integer(&mut self, buf: &mut String) -> Result<()> {}
    #[cfg(feature = "arbitrary_precision")]
    fn scan_number(&mut self, buf: &mut String) -> Result<()> {}
    #[cfg(feature = "arbitrary_precision")]
    fn scan_decimal(&mut self, buf: &mut String) -> Result<()> {}
    #[cfg(feature = "arbitrary_precision")]
    fn scan_exponent(&mut self, e: char, buf: &mut String) -> Result<()> {}
    fn parse_object_colon(&mut self) -> Result<()> {}
    fn end_seq(&mut self) -> Result<()> {}
    fn end_map(&mut self) -> Result<()> {}
    fn ignore_value(&mut self) -> Result<()> {}
    fn ignore_integer(&mut self) -> Result<()> {}
    fn ignore_decimal(&mut self) -> Result<()> {}
    fn ignore_exponent(&mut self) -> Result<()> {}
    #[cfg(feature = "raw_value")]
    fn deserialize_raw_value<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
}
