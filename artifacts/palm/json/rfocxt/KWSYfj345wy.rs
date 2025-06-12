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
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
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
pub(crate) trait FloatErrors {
    fn error_scale() -> u32;
    fn error_halfscale() -> u32;
    fn error_is_accurate<F: Float>(count: u32, fp: &ExtendedFloat) -> bool;
}
pub trait Float: Number {
    type Unsigned: Integer;
    const ZERO: Self;
    const MAX_DIGITS: usize;
    const EXPONENT_MASK: Self::Unsigned;
    const HIDDEN_BIT_MASK: Self::Unsigned;
    const MANTISSA_MASK: Self::Unsigned;
    const INFINITY_BITS: Self::Unsigned;
    const MANTISSA_SIZE: i32;
    const EXPONENT_BIAS: i32;
    const DENORMAL_EXPONENT: i32;
    const MAX_EXPONENT: i32;
    const DEFAULT_SHIFT: i32;
    const CARRY_MASK: u64;
    fn exponent_limit() -> (i32, i32);
    fn mantissa_limit() -> i32;
    fn pow10(self, n: i32) -> Self;
    fn from_bits(u: Self::Unsigned) -> Self;
    fn to_bits(self) -> Self::Unsigned;
    fn is_sign_positive(self) -> bool;
    #[inline]
    fn is_denormal(self) -> bool;
    #[inline]
    fn is_special(self) -> bool;
    #[inline]
    fn is_inf(self) -> bool;
    #[inline]
    fn exponent(self) -> i32;
    #[inline]
    fn mantissa(self) -> Self::Unsigned {
        let bits = self.to_bits();
        let s = bits & Self::MANTISSA_MASK;
        if !self.is_denormal() { s + Self::HIDDEN_BIT_MASK } else { s }
    }
    #[inline]
    fn next_positive(self) -> Self {
        debug_assert!(self.is_sign_positive() && ! self.is_inf());
        Self::from_bits(self.to_bits() + Self::Unsigned::as_cast(1u32))
    }
    #[inline]
    fn round_positive_even(self) -> Self {
        if self.mantissa() & Self::Unsigned::as_cast(1u32)
            == Self::Unsigned::as_cast(1u32)
        {
            self.next_positive()
        } else {
            self
        }
    }
}
trait Hi64<T>: AsRef<[T]> {
    fn hi64_1(&self) -> (u64, bool);
    fn hi64_2(&self) -> (u64, bool);
    fn hi64_3(&self) -> (u64, bool);
    #[inline]
    fn hi64(&self) -> (u64, bool);
}
pub trait Index: private::Sealed {
    fn index_into<'v>(&self, v: &'v Value) -> Option<&'v Value>;
    fn index_into_mut<'v>(&self, v: &'v mut Value) -> Option<&'v mut Value>;
    fn index_or_insert<'v>(&self, v: &'v mut Value) -> &'v mut Value;
}
pub struct Serializer;
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub struct Error;
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct Serializer<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
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
impl<'de, R: Read<'de>> de::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;
    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }
    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {}
}
