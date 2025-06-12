use crate::error::{Error, ErrorCode, Result};
use crate::io;
use alloc::string::String;
#[cfg(feature = "raw_value")]
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::{self, Display};
use core::num::FpCategory;
use serde::ser::{self, Impossible, Serialize};
static ESCAPE: [u8; 256] = [
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    BB,
    TT,
    NN,
    UU,
    FF,
    RR,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    UU,
    __,
    __,
    QU,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    BS,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
    __,
];
const BB: u8 = b'b';
const TT: u8 = b't';
const NN: u8 = b'n';
const FF: u8 = b'f';
const RR: u8 = b'r';
const QU: u8 = b'"';
const BS: u8 = b'\\';
const UU: u8 = b'u';
const __: u8 = 0;
pub trait Formatter {
    #[inline]
    fn write_null<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"null")
    }
    #[inline]
    fn write_bool<W>(&mut self, writer: &mut W, value: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let s = if value { b"true" as &[u8] } else { b"false" as &[u8] };
        writer.write_all(s)
    }
    #[inline]
    fn write_i8<W>(&mut self, writer: &mut W, value: i8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_i16<W>(&mut self, writer: &mut W, value: i16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_i32<W>(&mut self, writer: &mut W, value: i32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_i64<W>(&mut self, writer: &mut W, value: i64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_i128<W>(&mut self, writer: &mut W, value: i128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_u8<W>(&mut self, writer: &mut W, value: u8) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_u16<W>(&mut self, writer: &mut W, value: u16) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_u32<W>(&mut self, writer: &mut W, value: u32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_u64<W>(&mut self, writer: &mut W, value: u64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_u128<W>(&mut self, writer: &mut W, value: u128) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        let mut buffer = ryu::Buffer::new();
        let s = buffer.format_finite(value);
        writer.write_all(s.as_bytes())
    }
    #[inline]
    fn write_number_str<W>(&mut self, writer: &mut W, value: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(value.as_bytes())
    }
    #[inline]
    fn begin_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }
    #[inline]
    fn end_string<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"\"")
    }
    #[inline]
    fn write_string_fragment<W>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }
    #[inline]
    fn write_char_escape<W>(
        &mut self,
        writer: &mut W,
        char_escape: CharEscape,
    ) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        use self::CharEscape::*;
        let s = match char_escape {
            Quote => b"\\\"",
            ReverseSolidus => b"\\\\",
            Solidus => b"\\/",
            Backspace => b"\\b",
            FormFeed => b"\\f",
            LineFeed => b"\\n",
            CarriageReturn => b"\\r",
            Tab => b"\\t",
            AsciiControl(byte) => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                let bytes = &[
                    b'\\',
                    b'u',
                    b'0',
                    b'0',
                    HEX_DIGITS[(byte >> 4) as usize],
                    HEX_DIGITS[(byte & 0xF) as usize],
                ];
                return writer.write_all(bytes);
            }
        };
        writer.write_all(s)
    }
    fn write_byte_array<W>(&mut self, writer: &mut W, value: &[u8]) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(self.begin_array(writer));
        let mut first = true;
        for byte in value {
            tri!(self.begin_array_value(writer, first));
            tri!(self.write_u8(writer, * byte));
            tri!(self.end_array_value(writer));
            first = false;
        }
        self.end_array(writer)
    }
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"[")
    }
    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"]")
    }
    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first { Ok(()) } else { writer.write_all(b",") }
    }
    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"{")
    }
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b"}")
    }
    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if first { Ok(()) } else { writer.write_all(b",") }
    }
    #[inline]
    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b":")
    }
    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }
    #[inline]
    fn write_raw_fragment<W>(&mut self, writer: &mut W, fragment: &str) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(fragment.as_bytes())
    }
}
#[derive(Clone, Debug)]
pub struct PrettyFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
}
impl<'a> Formatter for PrettyFormatter<'a> {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }
    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {}
}
