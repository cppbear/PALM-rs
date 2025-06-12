//! When serializing or deserializing JSON goes wrong.

use crate::io;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::{self, Debug, Display};
use core::result;
use core::str::FromStr;
use serde::{de, ser};
#[cfg(feature = "std")]
use std::error;
#[cfg(feature = "std")]
use std::io::ErrorKind;

/// This type represents all possible errors that can occur when serializing or
/// deserializing JSON data.
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}

/// Alias for a `Result` with the error type `serde_json::Error`.
pub type Result<T> = result::Result<T, Error>;

impl Error {
    /// One-based line number at which the error was detected.
    ///
    /// Characters in the first line of the input (before the first newline
    /// character) are in line 1.
    pub fn line(&self) -> usize {
        self.err.line
    }

    /// One-based column number at which the error was detected.
    ///
    /// The first character in the input and any characters immediately
    /// following a newline character are in column 1.
    ///
    /// Note that errors may occur in column 0, for example if a read from an
    /// I/O stream fails immediately following a previously read newline
    /// character.
    pub fn column(&self) -> usize {
        self.err.column
    }

    /// Categorizes the cause of this error.
    ///
    /// - `Category::Io` - failure to read or write bytes on an I/O stream
    /// - `Category::Syntax` - input that is not syntactically valid JSON
    /// - `Category::Data` - input data that is semantically incorrect
    /// - `Category::Eof` - unexpected end of the input data
    pub fn classify(&self) -> Category {
        match self.err.code {
            ErrorCode::Message(_) => Category::Data,
            ErrorCode::Io(_) => Category::Io,
            ErrorCode::EofWhileParsingList
            | ErrorCode::EofWhileParsingObject
            | ErrorCode::EofWhileParsingString
            | ErrorCode::EofWhileParsingValue => Category::Eof,
            ErrorCode::ExpectedColon
            | ErrorCode::ExpectedListCommaOrEnd
            | ErrorCode::ExpectedObjectCommaOrEnd
            | ErrorCode::ExpectedSomeIdent
            | ErrorCode::ExpectedSomeValue
            | ErrorCode::ExpectedDoubleQuote
            | ErrorCode::InvalidEscape
            | ErrorCode::InvalidNumber
            | ErrorCode::NumberOutOfRange
            | ErrorCode::InvalidUnicodeCodePoint
            | ErrorCode::ControlCharacterWhileParsingString
            | ErrorCode::KeyMustBeAString
            | ErrorCode::ExpectedNumericKey
            | ErrorCode::FloatKeyMustBeFinite
            | ErrorCode::LoneLeadingSurrogateInHexEscape
            | ErrorCode::TrailingComma
            | ErrorCode::TrailingCharacters
            | ErrorCode::UnexpectedEndOfHexEscape
            | ErrorCode::RecursionLimitExceeded => Category::Syntax,
        }
    }

    /// Returns true if this error was caused by a failure to read or write
    /// bytes on an I/O stream.
    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }

    /// Returns true if this error was caused by input that was not
    /// syntactically valid JSON.
    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }

    /// Returns true if this error was caused by input data that was
    /// semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    pub fn is_data(&self) -> bool {
        self.classify() == Category::Data
    }

    /// Returns true if this error was caused by prematurely reaching the end of
    /// the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }

    /// The kind reported by the underlying standard library I/O error, if this
    /// error was caused by a failure to read or write bytes on an I/O stream.
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::Value;
    /// use std::io::{self, ErrorKind, Read};
    /// use std::process;
    ///
    /// struct ReaderThatWillTimeOut<'a>(&'a [u8]);
    ///
    /// impl<'a> Read for ReaderThatWillTimeOut<'a> {
    ///     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    ///         if self.0.is_empty() {
    ///             Err(io::Error::new(ErrorKind::TimedOut, "timed out"))
    ///         } else {
    ///             self.0.read(buf)
    ///         }
    ///     }
    /// }
    ///
    /// fn main() {
    ///     let reader = ReaderThatWillTimeOut(br#" {"k": "#);
    ///
    ///     let _: Value = match serde_json::from_reader(reader) {
    ///         Ok(value) => value,
    ///         Err(error) => {
    ///             if error.io_error_kind() == Some(ErrorKind::TimedOut) {
    ///                 // Maybe this application needs to retry certain kinds of errors.
    ///
    ///                 # return;
    ///             } else {
    ///                 eprintln!("error: {}", error);
    ///                 process::exit(1);
    ///             }
    ///         }
    ///     };
    /// }
    /// ```
    #[cfg(feature = "std")]
    pub fn io_error_kind(&self) -> Option<ErrorKind> {
        if let ErrorCode::Io(io_error) = &self.err.code {
            Some(io_error.kind())
        } else {
            None
        }
    }
}

/// Categorizes the cause of a `serde_json::Error`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Category {
    /// The error was caused by a failure to read or write bytes on an I/O
    /// stream.
    Io,

    /// The error was caused by input that was not syntactically valid JSON.
    Syntax,

    /// The error was caused by input data that was semantically incorrect.
    ///
    /// For example, JSON containing a number is semantically incorrect when the
    /// type being deserialized into holds a String.
    Data,

    /// The error was caused by prematurely reaching the end of the input data.
    ///
    /// Callers that process streaming input may be interested in retrying the
    /// deserialization once more data is available.
    Eof,
}

#[cfg(feature = "std")]
#[allow(clippy::fallible_impl_from)]
impl From<Error> for io::Error {
    /// Convert a `serde_json::Error` into an `io::Error`.
    ///
    /// JSON syntax and data errors are turned into `InvalidData` I/O errors.
    /// EOF errors are turned into `UnexpectedEof` I/O errors.
    ///
    /// ```
    /// use std::io;
    ///
    /// enum MyError {
    ///     Io(io::Error),
    ///     Json(serde_json::Error),
    /// }
    ///
    /// impl From<serde_json::Error> for MyError {
    ///     fn from(err: serde_json::Error) -> MyError {
    ///         use serde_json::error::Category;
    ///         match err.classify() {
    ///             Category::Io => {
    ///                 MyError::Io(err.into())
    ///             }
    ///             Category::Syntax | Category::Data | Category::Eof => {
    ///                 MyError::Json(err)
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    fn from(j: Error) -> Self {
        if let ErrorCode::Io(err) = j.err.code {
            err
        } else {
            match j.classify() {
                Category::Io => unreachable!(),
                Category::Syntax | Category::Data => io::Error::new(ErrorKind::InvalidData, j),
                Category::Eof => io::Error::new(ErrorKind::UnexpectedEof, j),
            }
        }
    }
}

struct ErrorImpl {
    code: ErrorCode,
    line: usize,
    column: usize,
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

impl Error {
    #[cold]
    pub(crate) fn syntax(code: ErrorCode, line: usize, column: usize) -> Self {
        Error {
            err: Box::new(ErrorImpl { code, line, column }),
        }
    }

    // Not public API. Should be pub(crate).
    //
    // Update `eager_json` crate when this function changes.
    #[doc(hidden)]
    #[cold]
    pub fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                line: 0,
                column: 0,
            }),
        }
    }

    #[cold]
    pub(crate) fn fix_position<F>(self, f: F) -> Self
    where
        F: FnOnce(ErrorCode) -> Error,
    {
        if self.err.line == 0 {
            f(self.err.code)
        } else {
            self
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, f),
            ErrorCode::EofWhileParsingList => f.write_str("EOF while parsing a list"),
            ErrorCode::EofWhileParsingObject => f.write_str("EOF while parsing an object"),
            ErrorCode::EofWhileParsingString => f.write_str("EOF while parsing a string"),
            ErrorCode::EofWhileParsingValue => f.write_str("EOF while parsing a value"),
            ErrorCode::ExpectedColon => f.write_str("expected `:`"),
            ErrorCode::ExpectedListCommaOrEnd => f.write_str("expected `,` or `]`"),
            ErrorCode::ExpectedObjectCommaOrEnd => f.write_str("expected `,` or `}`"),
            ErrorCode::ExpectedSomeIdent => f.write_str("expected ident"),
            ErrorCode::ExpectedSomeValue => f.write_str("expected value"),
            ErrorCode::ExpectedDoubleQuote => f.write_str("expected `\"`"),
            ErrorCode::InvalidEscape => f.write_str("invalid escape"),
            ErrorCode::InvalidNumber => f.write_str("invalid number"),
            ErrorCode::NumberOutOfRange => f.write_str("number out of range"),
            ErrorCode::InvalidUnicodeCodePoint => f.write_str("invalid unicode code point"),
            ErrorCode::ControlCharacterWhileParsingString => {
                f.write_str("control character (\\u0000-\\u001F) found while parsing a string")
            }
            ErrorCode::KeyMustBeAString => f.write_str("key must be a string"),
            ErrorCode::ExpectedNumericKey => {
                f.write_str("invalid value: expected key to be a number in quotes")
            }
            ErrorCode::FloatKeyMustBeFinite => {
                f.write_str("float key must be finite (got NaN or +/-inf)")
            }
            ErrorCode::LoneLeadingSurrogateInHexEscape => {
                f.write_str("lone leading surrogate in hex escape")
            }
            ErrorCode::TrailingComma => f.write_str("trailing comma"),
            ErrorCode::TrailingCharacters => f.write_str("trailing characters"),
            ErrorCode::UnexpectedEndOfHexEscape => f.write_str("unexpected end of hex escape"),
            ErrorCode::RecursionLimitExceeded => f.write_str("recursion limit exceeded"),
        }
    }
}

impl serde::de::StdError for Error {
    #[cfg(feature = "std")]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.err.code {
            ErrorCode::Io(err) => err.source(),
            _ => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.line == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(
                f,
                "{} at line {} column {}",
                self.code, self.line, self.column
            )
        }
    }
}

// Remove two layers of verbosity from the debug representation. Humans often
// end up seeing this representation because it is what unwrap() shows.
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error({:?}, line: {}, column: {})",
            self.err.code.to_string(),
            self.err.line,
            self.err.column
        )
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }

    #[cold]
    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!(
            "invalid type: {}, expected {}",
            JsonUnexpected(unexp),
            exp,
        ))
    }

    #[cold]
    fn invalid_value(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        Error::custom(format_args!(
            "invalid value: {}, expected {}",
            JsonUnexpected(unexp),
            exp,
        ))
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Error {
        make_error(msg.to_string())
    }
}

struct JsonUnexpected<'a>(de::Unexpected<'a>);

impl<'a> Display for JsonUnexpected<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            de::Unexpected::Unit => formatter.write_str("null"),
            de::Unexpected::Float(value) => write!(
                formatter,
                "floating point `{}`",
                ryu::Buffer::new().format(value),
            ),
            unexp => Display::fmt(&unexp, formatter),
        }
    }
}

// Parse our own error message that looks like "{} at line {} column {}" to work
// around erased-serde round-tripping the error through de::Error::custom.
fn make_error(mut msg: String) -> Error {
    let (line, column) = parse_line_col(&mut msg).unwrap_or((0, 0));
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg.into_boxed_str()),
            line,
            column,
        }),
    }
}

fn parse_line_col(msg: &mut String) -> Option<(usize, usize)> {
    let start_of_suffix = match msg.rfind(" at line ") {
        Some(index) => index,
        None => return None,
    };

    // Find start and end of line number.
    let start_of_line = start_of_suffix + " at line ".len();
    let mut end_of_line = start_of_line;
    while starts_with_digit(&msg[end_of_line..]) {
        end_of_line += 1;
    }

    if !msg[end_of_line..].starts_with(" column ") {
        return None;
    }

    // Find start and end of column number.
    let start_of_column = end_of_line + " column ".len();
    let mut end_of_column = start_of_column;
    while starts_with_digit(&msg[end_of_column..]) {
        end_of_column += 1;
    }

    if end_of_column < msg.len() {
        return None;
    }

    // Parse numbers.
    let line = match usize::from_str(&msg[start_of_line..end_of_line]) {
        Ok(line) => line,
        Err(_) => return None,
    };
    let column = match usize::from_str(&msg[start_of_column..end_of_column]) {
        Ok(column) => column,
        Err(_) => return None,
    };

    msg.truncate(start_of_suffix);
    Some((line, column))
}

fn starts_with_digit(slice: &str) -> bool {
    match slice.as_bytes().first() {
        None => false,
        Some(&byte) => byte >= b'0' && byte <= b'9',
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::iter::IntoIterator;
	use std::iter::FromIterator;
	use std::iter::ExactSizeIterator;
	use std::default::Default;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::str::FromStr;
	use std::error::Error;
	use std::iter::Iterator;
	use read::Read;
	use std::iter::DoubleEndedIterator;
	use std::cmp::Eq;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_0() {
    rusty_monitor::set_test_id(0);
    let mut str_0: &str = "INrlFfCIkpww";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut usize_0: usize = 9044usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut str_1: &str = "cQgJcEosX1y";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
    let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
    let mut str_2: &str = "IrI2j7VrqDi2nvvqdF";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut usize_1: usize = 4442usize;
    let mut usize_2: usize = 946usize;
    let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::KeyMustBeAString;
    let mut error_0: crate::error::Error = crate::error::Error::syntax(errorcode_0, usize_2, usize_1);
    let mut error_0_ref_0: &crate::error::Error = &mut error_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_1_ref_0);
    let mut keys_0_ref_0: &crate::map::Keys = &mut keys_0;
    let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::EofWhileParsingObject;
    let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Keys::size_hint(keys_0_ref_0);
    let mut errorcode_1_ref_0: &error::ErrorCode = &mut errorcode_1;
    let mut usize_3: usize = crate::error::Error::line(error_0_ref_0);
    let mut bool_0: bool = crate::error::starts_with_digit(str_2_ref_0);
    crate::read::StrRead::discard(strread_1_ref_0);
    let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
    let mut errorcode_2: error::ErrorCode = crate::error::ErrorCode::NumberOutOfRange;
    let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
    let mut option_0: std::option::Option<&value::Value> = crate::map::Values::next(values_0_ref_0);
    crate::read::StrRead::discard(strread_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut usize_0: usize = 655usize;
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
    let mut values_0_ref_0: &crate::map::Values = &mut values_0;
    let mut str_0: &str = "FKmpy";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
    let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
    let mut category_0: error::Category = crate::error::Category::Eof;
    let mut category_0_ref_0: &error::Category = &mut category_0;
    let mut option_0: std::option::Option<std::string::String> = std::option::Option::None;
    let mut usize_1: usize = 5266usize;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
    let mut str_1: &str = "UHGKA8bcN8Ik5nhe9iF";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
    let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
    let mut usize_2: usize = 3941usize;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_2);
    let mut intoiter_0: crate::map::IntoIter = crate::map::Map::into_iter(map_2);
    let mut intoiter_0_ref_0: &mut crate::map::IntoIter = &mut intoiter_0;
    let mut bool_0: bool = true;
    let mut u64_0: u64 = 8209u64;
    let mut option_1: std::option::Option<(std::string::String, value::Value)> = crate::map::IntoIter::next_back(intoiter_0_ref_0);
    let mut tuple_0: (std::string::String, value::Value) = std::option::Option::unwrap(option_1);
    crate::read::StrRead::next(strread_1_ref_0);
    let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::TrailingComma;
    let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::FloatKeyMustBeFinite;
    let mut errorcode_1_ref_0: &error::ErrorCode = &mut errorcode_1;
    let mut serializemap_0: value::ser::SerializeMap = crate::value::ser::SerializeMap::Map {map: map_1, next_key: option_0};
    let mut errorcode_0_ref_0: &error::ErrorCode = &mut errorcode_0;
    let mut errorcode_2: error::ErrorCode = crate::error::ErrorCode::InvalidUnicodeCodePoint;
    let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::Values::size_hint(values_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_14() {
    rusty_monitor::set_test_id(14);
    let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
    let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
    let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
    let mut str_0: &str = "l9C";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut usize_0: usize = 4500usize;
    let mut usize_1: usize = 3913usize;
    let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::TrailingCharacters;
    let mut error_0: crate::error::Error = crate::error::Error::syntax(errorcode_0, usize_1, usize_0);
    let mut error_0_ref_0: &crate::error::Error = &mut error_0;
    let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
    let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_1_ref_0);
    let mut iter_0_ref_0: &mut crate::map::Iter = &mut iter_0;
    let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
    let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
    let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_2_ref_0);
    let mut keys_0_ref_0: &mut crate::map::Keys = &mut keys_0;
    let mut option_0: std::option::Option<&std::string::String> = crate::map::Keys::next(keys_0_ref_0);
    let mut option_1: std::option::Option<(&std::string::String, &value::Value)> = crate::map::Iter::next(iter_0_ref_0);
    let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::EofWhileParsingString;
    let mut errorcode_1_ref_0: &error::ErrorCode = &mut errorcode_1;
    let mut string_0: &std::string::String = std::option::Option::unwrap(option_0);
    let mut errorcode_2: error::ErrorCode = crate::error::ErrorCode::EofWhileParsingObject;
    let mut option_2: std::option::Option<&dyn std::error::Error> = crate::error::Error::source(error_0_ref_0);
    let mut tuple_0: (&std::string::String, &value::Value) = std::option::Option::unwrap(option_1);
    let mut result_0: std::result::Result<crate::number::Number, crate::error::Error> = crate::number::Number::from_str(str_0_ref_0);
    let mut option_3: std::option::Option<&value::Value> = crate::map::Values::next_back(values_0_ref_0);
    let mut errorcode_3: error::ErrorCode = crate::error::ErrorCode::ControlCharacterWhileParsingString;
    panic!("From RustyUnit with love");
}

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_32() {
//     rusty_monitor::set_test_id(32);
//     let mut bool_0: bool = true;
//     let mut usize_0: usize = 8242usize;
//     let mut usize_1: usize = 5896usize;
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::InvalidNumber;
//     let mut error_0: crate::error::Error = crate::error::Error::syntax(errorcode_0, usize_1, usize_0);
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut usize_2: usize = 7199usize;
//     let mut usize_3: usize = 5014usize;
//     let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::LoneLeadingSurrogateInHexEscape;
//     let mut error_1: crate::error::Error = crate::error::Error::syntax(errorcode_1, usize_3, usize_2);
//     let mut error_1_ref_0: &crate::error::Error = &mut error_1;
//     let mut state_0: ser::State = crate::ser::State::Rest;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut usize_4: usize = 3117usize;
//     let mut usize_5: usize = 2637usize;
//     let mut errorcode_2: error::ErrorCode = crate::error::ErrorCode::ExpectedColon;
//     let mut error_2: crate::error::Error = crate::error::Error::syntax(errorcode_2, usize_5, usize_4);
//     let mut error_2_ref_0: &crate::error::Error = &mut error_2;
//     let mut category_0: error::Category = crate::error::Error::classify(error_2_ref_0);
//     let mut category_0_ref_0: &error::Category = &mut category_0;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_0_ref_0);
//     let mut values_0_ref_0: &crate::map::Values = &mut values_0;
//     let mut usize_6: usize = crate::map::Values::len(values_0_ref_0);
//     let mut errorcode_3: error::ErrorCode = crate::error::ErrorCode::FloatKeyMustBeFinite;
//     let mut errorcode_3_ref_0: &error::ErrorCode = &mut errorcode_3;
//     let mut tuple_0: () = crate::ser::State::assert_receiver_is_total_eq(state_0_ref_0);
//     let mut option_0: std::option::Option<std::io::ErrorKind> = crate::error::Error::io_error_kind(error_1_ref_0);
//     let mut value_0: value::Value = crate::value::Value::Bool(bool_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_34() {
//     rusty_monitor::set_test_id(34);
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut usize_0: usize = 6744usize;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_2_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_2_ref_0);
//     let mut iter_0_ref_0: &crate::map::Iter = &mut iter_0;
//     let mut category_0: error::Category = crate::error::Category::Eof;
//     let mut category_0_ref_0: &error::Category = &mut category_0;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_3_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_3;
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::iter_mut(map_3_ref_0);
//     let mut itermut_0_ref_0: &crate::map::IterMut = &mut itermut_0;
//     let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_4_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_4;
//     let mut usize_1: usize = 4639usize;
//     let mut map_5: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
//     let mut box_0: std::boxed::Box<crate::error::ErrorImpl> = std::boxed::Box::new();
//     let mut error_0: crate::error::Error = crate::error::Error {err: box_0};
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::ExpectedListCommaOrEnd;
//     let mut value_0: value::Value = crate::value::Value::Object(map_5);
//     let mut state_0: ser::State = crate::ser::State::Empty;
//     let mut iter_1: crate::map::Iter = crate::map::Map::into_iter(map_4_ref_0);
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::IterMut::size_hint(itermut_0_ref_0);
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut category_1: error::Category = crate::error::Error::classify(error_0_ref_0);
//     let mut iter_1_ref_0: &crate::map::Iter = &mut iter_1;
//     let mut tuple_1: (usize, std::option::Option<usize>) = crate::map::Iter::size_hint(iter_1_ref_0);
//     let mut usize_2: usize = crate::map::Iter::len(iter_0_ref_0);
//     let mut category_1_ref_0: &error::Category = &mut category_1;
//     let mut category_2: error::Category = crate::error::Category::clone(category_1_ref_0);
//     let mut errorcode_0_ref_0: &error::ErrorCode = &mut errorcode_0;
//     let mut bool_0: bool = crate::map::Map::eq(map_1_ref_0, map_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_37() {
//     rusty_monitor::set_test_id(37);
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut usize_0: usize = 9627usize;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut category_0: error::Category = crate::error::Category::Eof;
//     let mut category_0_ref_0: &error::Category = &mut category_0;
//     let mut str_0: &str = "Cv";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut usize_1: usize = 9065usize;
//     let mut usize_2: usize = 4557usize;
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::InvalidUnicodeCodePoint;
//     let mut bool_0: bool = false;
//     let mut usize_3: usize = 3025usize;
//     let mut usize_4: usize = 5947usize;
//     let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::ExpectedDoubleQuote;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::from_iter::<std::vec::Vec<u8>>(vec_0);
//     let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_2);
//     let mut intovalues_0_ref_0: &mut crate::map::IntoValues = &mut intovalues_0;
//     let mut bool_1: bool = true;
//     let mut value_0: value::Value = crate::value::Value::Bool(bool_1);
//     let mut option_0: std::option::Option<value::Value> = crate::map::IntoValues::next(intovalues_0_ref_0);
//     let mut errorcode_2: error::ErrorCode = crate::error::ErrorCode::ExpectedSomeValue;
//     let mut errorimpl_0: crate::error::ErrorImpl = crate::error::ErrorImpl {code: errorcode_1, line: usize_4, column: usize_3};
//     let mut value_1: value::Value = crate::value::Value::Bool(bool_0);
//     let mut errorimpl_1: crate::error::ErrorImpl = crate::error::ErrorImpl {code: errorcode_0, line: usize_2, column: usize_1};
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::Quote;
//     let mut tuple_0: () = crate::error::Category::assert_receiver_is_total_eq(category_0_ref_0);
//     let mut value_2: value::Value = std::option::Option::unwrap(option_0);
//     let mut bool_2: bool = crate::map::Map::eq(map_1_ref_0, map_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_39() {
//     rusty_monitor::set_test_id(39);
//     let mut usize_0: usize = 2431usize;
//     let mut usize_1: usize = 9312usize;
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::TrailingComma;
//     let mut error_0: crate::error::Error = crate::error::Error::syntax(errorcode_0, usize_1, usize_0);
//     let mut error_0_ref_0: &crate::error::Error = &mut error_0;
//     let mut str_0: &str = "au4YQyI6ly0eE";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut bool_0: bool = true;
//     let mut bool_0_ref_0: &mut bool = &mut bool_0;
//     let mut usize_2: usize = 2469usize;
//     let mut vec_0: std::vec::Vec<value::Value> = std::vec::Vec::new();
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::from_iter::<std::vec::Vec<value::Value>>(vec_0);
//     let mut map_0_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut str_1: &str = "8e3Qih1FXMdb";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
//     let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
//     let mut usize_3: usize = 5570usize;
//     let mut usize_4: usize = 2120usize;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_4);
//     let mut map_1_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut valuesmut_0: crate::map::ValuesMut = crate::map::Map::values_mut(map_1_ref_0);
//     let mut valuesmut_0_ref_0: &crate::map::ValuesMut = &mut valuesmut_0;
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::ValuesMut::size_hint(valuesmut_0_ref_0);
//     crate::read::StrRead::peek(strread_1_ref_0);
//     let mut itermut_0: crate::map::IterMut = crate::map::Map::into_iter(map_0_ref_0);
//     let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::InvalidEscape;
//     let mut itermut_0_ref_0: &mut crate::map::IterMut = &mut itermut_0;
//     let mut option_0: std::option::Option<(&std::string::String, &mut value::Value)> = crate::map::IterMut::next(itermut_0_ref_0);
//     crate::read::StrRead::ignore_str(strread_0_ref_0);
//     let mut option_1: std::option::Option<&dyn std::error::Error> = crate::error::Error::source(error_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_43() {
//     rusty_monitor::set_test_id(43);
//     let mut state_0: ser::State = crate::ser::State::Rest;
//     let mut state_0_ref_0: &ser::State = &mut state_0;
//     let mut state_1: ser::State = crate::ser::State::First;
//     let mut state_1_ref_0: &ser::State = &mut state_1;
//     let mut usize_0: usize = 6520usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut iter_0: crate::map::Iter = crate::map::Map::iter(map_0_ref_0);
//     let mut iter_0_ref_0: &mut crate::map::Iter = &mut iter_0;
//     let mut usize_1: usize = 2616usize;
//     let mut box_0: std::boxed::Box<&str> = std::boxed::Box::new();
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut keys_0: crate::map::Keys = crate::map::Map::keys(map_1_ref_0);
//     let mut keys_0_ref_0: &crate::map::Keys = &mut keys_0;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut intovalues_0: crate::map::IntoValues = crate::map::Map::into_values(map_2);
//     let mut intovalues_0_ref_0: &crate::map::IntoValues = &mut intovalues_0;
//     let mut map_3: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_3_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_3;
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_3_ref_0);
//     let mut values_0_ref_0: &mut crate::map::Values = &mut values_0;
//     let mut option_0: std::option::Option<&value::Value> = crate::map::Values::next_back(values_0_ref_0);
//     let mut usize_2: usize = crate::map::IntoValues::len(intovalues_0_ref_0);
//     let mut tuple_0: (usize, std::option::Option<usize>) = crate::map::Keys::size_hint(keys_0_ref_0);
//     let mut value_0: &value::Value = std::option::Option::unwrap(option_0);
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::Message(box_0);
//     let mut errorcode_0_ref_0: &error::ErrorCode = &mut errorcode_0;
//     let mut map_4: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_1);
//     let mut option_1: std::option::Option<(&std::string::String, &value::Value)> = crate::map::Iter::next(iter_0_ref_0);
//     let mut bool_0: bool = crate::ser::State::eq(state_1_ref_0, state_0_ref_0);
//     panic!("From RustyUnit with love");
// }

// #[test]
// #[should_panic]
// #[timeout(3000)]
// fn rusty_test_47() {
//     rusty_monitor::set_test_id(47);
//     let mut usize_0: usize = 1721usize;
//     let mut map_0: crate::map::Map<std::string::String, value::Value> = crate::map::Map::with_capacity(usize_0);
//     let mut map_0_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_0;
//     let mut iter_0: crate::map::Iter = crate::map::Map::into_iter(map_0_ref_0);
//     let mut iter_0_ref_0: &mut crate::map::Iter = &mut iter_0;
//     let mut map_1: crate::map::Map<std::string::String, value::Value> = crate::map::Map::default();
//     let mut map_1_ref_0: &crate::map::Map<std::string::String, value::Value> = &mut map_1;
//     let mut str_0: &str = "Zc0jl";
//     let mut str_0_ref_0: &str = &mut str_0;
//     let mut strread_0: crate::read::StrRead = crate::read::StrRead::new(str_0_ref_0);
//     let mut strread_0_ref_0: &mut crate::read::StrRead = &mut strread_0;
//     let mut errorcode_0: error::ErrorCode = crate::error::ErrorCode::ExpectedSomeValue;
//     let mut errorcode_0_ref_0: &error::ErrorCode = &mut errorcode_0;
//     let mut map_2: crate::map::Map<std::string::String, value::Value> = crate::map::Map::new();
//     let mut map_2_ref_0: &mut crate::map::Map<std::string::String, value::Value> = &mut map_2;
//     let mut bool_0: bool = false;
//     let mut u8_0: u8 = 21u8;
//     let mut isize_0: isize = -13729isize;
//     let mut vec_0: std::vec::Vec<u8> = std::vec::Vec::new();
//     let mut vec_0_ref_0: &mut std::vec::Vec<u8> = &mut vec_0;
//     let mut str_1: &str = "";
//     let mut str_1_ref_0: &str = &mut str_1;
//     let mut strread_1: crate::read::StrRead = crate::read::StrRead::new(str_1_ref_0);
//     let mut strread_1_ref_0: &mut crate::read::StrRead = &mut strread_1;
//     crate::read::StrRead::parse_str(strread_1_ref_0, vec_0_ref_0);
//     crate::map::Map::clear(map_2_ref_0);
//     crate::read::StrRead::decode_hex_escape(strread_0_ref_0);
//     let mut errorcode_1: error::ErrorCode = crate::error::ErrorCode::EofWhileParsingValue;
//     let mut values_0: crate::map::Values = crate::map::Map::values(map_1_ref_0);
//     let mut charescape_0: ser::CharEscape = crate::ser::CharEscape::LineFeed;
//     let mut option_0: std::option::Option<(&std::string::String, &value::Value)> = crate::map::Iter::next_back(iter_0_ref_0);
//     panic!("From RustyUnit with love");
// }

}