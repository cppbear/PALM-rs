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
mod tests_llm_16_611 {
    use super::*;

use crate::*;
    use std::io;

    #[test]
    fn test_classify_data_category() {
        let error = Error::syntax(ErrorCode::Message("test".into()), 1, 1);
        assert_eq!(error.classify(), Category::Data);
    }

    #[test]
    fn test_classify_io_category() {
        let io_error = io::Error::new(io::ErrorKind::Other, "io error");
        let error = Error::io(io_error);
        assert_eq!(error.classify(), Category::Io);
    }

    #[test]
    fn test_classify_eof_category() {
        let error = Error::syntax(ErrorCode::EofWhileParsingObject, 1, 1);
        assert_eq!(error.classify(), Category::Eof);
    }

    #[test]
    fn test_classify_syntax_category() {
        let error = Error::syntax(ErrorCode::ExpectedColon, 1, 1);
        assert_eq!(error.classify(), Category::Syntax);
    }
}

#[cfg(test)]
mod tests_llm_16_612 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorCode};

    #[test]
    fn test_column() {
        let err = Error::syntax(ErrorCode::ExpectedDoubleQuote, 3, 5);
        assert_eq!(err.column(), 5);
    }

    #[test]
    fn test_column_zero() {
        let err = Error::io(std::io::Error::new(std::io::ErrorKind::Other, "error"));
        assert_eq!(err.column(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_613 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorCode};

    #[test]
    fn test_fix_position_no_line() {
        let error = Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0);
        let fixed_error = error.fix_position(|code| Error::syntax(code, 1, 1));
        assert_eq!(fixed_error.line(), 1);
        assert_eq!(fixed_error.column(), 1);
    }

    #[test]
    fn test_fix_position_with_line() {
        let error = Error::syntax(ErrorCode::ExpectedSomeValue, 1, 1);
        let fixed_error = error.fix_position(|code| Error::syntax(code, 2, 2));
        assert_eq!(fixed_error.line(), 1);
        assert_eq!(fixed_error.column(), 1);
    }

    #[test]
    fn test_fix_position_with_different_error_code() {
        let error = Error::syntax(ErrorCode::ExpectedSomeValue, 0, 0);
        let fixed_error = error.fix_position(|code| Error::syntax(ErrorCode::InvalidNumber, 3, 4));
        assert_eq!(fixed_error.line(), 3);
        assert_eq!(fixed_error.column(), 4);
    }
}

#[cfg(test)]
mod tests_llm_16_614 {
    use super::*;

use crate::*;
    use std::io::{self, ErrorKind};

    #[test]
    fn test_error_io_creation() {
        let io_error = io::Error::new(ErrorKind::NotFound, "file not found");
        let serde_error = Error::io(io_error);

        assert_eq!(serde_error.classify(), Category::Io);
        assert!(serde_error.is_io());
        assert!(!serde_error.is_syntax());
        assert!(!serde_error.is_data());
        assert!(!serde_error.is_eof());
        assert_eq!(serde_error.line(), 0);
        assert_eq!(serde_error.column(), 0);
    }

    #[test]
    fn test_io_error_kind() {
        let io_error = io::Error::new(ErrorKind::PermissionDenied, "permission denied");
        let serde_error = Error::io(io_error);
        assert_eq!(serde_error.io_error_kind(), Some(ErrorKind::PermissionDenied));
    }
}

#[cfg(test)]
mod tests_llm_16_615 {
    use super::*;

use crate::*;
    use std::io::{self, ErrorKind};
    
    #[test]
    fn test_io_error_kind_some() {
        let io_error = io::Error::new(ErrorKind::TimedOut, "timed out");
        let error = Error::io(io_error);
        assert_eq!(error.io_error_kind(), Some(ErrorKind::TimedOut));
    }

    #[test]
    fn test_io_error_kind_none() {
        let error = Error::syntax(ErrorCode::ExpectedColon, 1, 1);
        assert_eq!(error.io_error_kind(), None);
    }
    
    #[test]
    fn test_io_error_kind_other_kind() {
        let io_error = io::Error::new(ErrorKind::Other, "some error");
        let error = Error::io(io_error);
        assert_eq!(error.io_error_kind(), Some(ErrorKind::Other));
    }
}

#[cfg(test)]
mod tests_llm_16_617 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorCode, Category};

    #[test]
    fn test_is_eof() {
        let eof_error = Error::syntax(ErrorCode::EofWhileParsingValue, 1, 1);
        assert!(eof_error.is_eof());

        let syntax_error = Error::syntax(ErrorCode::ExpectedColon, 1, 1);
        assert!(!syntax_error.is_eof());

        let io_error = Error::io(std::io::Error::new(std::io::ErrorKind::Other, "error"));
        assert!(!io_error.is_eof());
    }
}

#[cfg(test)]
mod tests_llm_16_619 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorCode};

    #[test]
    fn test_is_syntax() {
        let syntax_error = Error::syntax(ErrorCode::ExpectedSomeValue, 1, 5);
        assert!(syntax_error.is_syntax());

        let io_error = Error::io(std::io::Error::new(std::io::ErrorKind::Other, "test"));
        assert!(!io_error.is_syntax());

        let data_error = Error::syntax(ErrorCode::InvalidNumber, 2, 10);
        assert!(data_error.is_syntax());

        let eof_error = Error::syntax(ErrorCode::EofWhileParsingValue, 3, 0);
        assert!(eof_error.is_syntax());
    }
}

#[cfg(test)]
mod tests_llm_16_620 {
    use super::*;

use crate::*;
    use crate::error::{Error, ErrorCode};

    #[test]
    fn test_line() {
        let error = Error::syntax(ErrorCode::ExpectedSomeValue, 5, 10);
        assert_eq!(error.line(), 5);
        
        let error = Error::syntax(ErrorCode::InvalidNumber, 3, 15);
        assert_eq!(error.line(), 3);
        
        let error = Error::io(std::io::Error::new(std::io::ErrorKind::Other, "Error"));
        assert_eq!(error.line(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_621 {
    use super::*;

use crate::*;
    use std::io;

    #[test]
    fn test_syntax_error() {
        let error_code = ErrorCode::ExpectedColon;
        let line = 10;
        let column = 5;

        let error = Error::syntax(error_code, line, column);

        assert_eq!(error.line(), line);
        assert_eq!(error.column(), column);
        assert!(error.is_syntax());
    }

    #[test]
    fn test_syntax_error_message() {
        let error_code = ErrorCode::Message("This is a syntax error".into());
        let line = 2;
        let column = 3;

        let error = Error::syntax(error_code, line, column);

        assert_eq!(error.line(), line);
        assert_eq!(error.column(), column);
        assert!(error.is_data());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = io::Error::new(io::ErrorKind::Other, "IO error occurred");
        let error = Error::io(io_error);
        let converted: io::Error = error.into();
        
        assert_eq!(converted.kind(), io::ErrorKind::InvalidData);
    }
}

#[cfg(test)]
mod tests_llm_16_622 {
    use super::*;

use crate::*;
    use crate::Error;

    #[test]
    fn test_make_error_with_valid_message() {
        let msg = String::from("This is a test error message");
        let error = make_error(msg.clone());

        assert_eq!(error.to_string(), msg);
        assert_eq!(error.line(), 0);
        assert_eq!(error.column(), 0);
        assert!(error.is_data());
    }

    #[test]
    fn test_make_error_with_empty_message() {
        let msg = String::from("");
        let error = make_error(msg.clone());

        assert_eq!(error.to_string(), msg);
        assert_eq!(error.line(), 0);
        assert_eq!(error.column(), 0);
        assert!(error.is_data());
    }

    #[test]
    fn test_make_error_handles_line_col() {
        let msg = String::from("Error at line 5, column 10");
        let error = make_error(msg.clone());

        assert_eq!(error.to_string(), msg);
        assert_eq!(error.line(), 0); // Assuming `parse_line_col` returns (0, 0) for this message
        assert_eq!(error.column(), 0); // Assuming `parse_line_col` returns (0, 0) for this message
        assert!(error.is_data());
    }
}

#[cfg(test)]
mod tests_llm_16_623 {
    use super::*;

use crate::*;
    use crate::error::parse_line_col;

    #[test]
    fn test_parse_line_col_valid() {
        let mut msg = String::from("Error occurred at line 10 column 5");
        let result = parse_line_col(&mut msg);
        assert_eq!(result, Some((10, 5)));
        assert_eq!(msg, "Error occurred ");
    }

    #[test]
    fn test_parse_line_col_no_line() {
        let mut msg = String::from("Error occurred somewhere");
        let result = parse_line_col(&mut msg);
        assert_eq!(result, None);
        assert_eq!(msg, "Error occurred somewhere");
    }

    #[test]
    fn test_parse_line_col_invalid_format() {
        let mut msg = String::from("Error at line 10 but no column");
        let result = parse_line_col(&mut msg);
        assert_eq!(result, None);
        assert_eq!(msg, "Error at line 10 but no column");
    }

    #[test]
    fn test_parse_line_col_invalid_numbers() {
        let mut msg = String::from("Error occurred at line foo column bar");
        let result = parse_line_col(&mut msg);
        assert_eq!(result, None);
        assert_eq!(msg, "Error occurred at line foo column bar");
    }

    #[test]
    fn test_parse_line_col_edge_case() {
        let mut msg = String::from("Error occurred at line 0 column 0");
        let result = parse_line_col(&mut msg);
        assert_eq!(result, Some((0, 0)));
        assert_eq!(msg, "Error occurred ");
    }
}

#[cfg(test)]
mod tests_llm_16_624 {
    use super::*;

use crate::*;

    #[test]
    fn test_starts_with_digit() {
        assert!(starts_with_digit("1abc"));
        assert!(starts_with_digit("0test"));
        assert!(!starts_with_digit("abc"));
        assert!(!starts_with_digit("!@#"));
        assert!(!starts_with_digit(""));
    }
}
