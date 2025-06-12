pub type Result<T> = result::Result<T, Error>;
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
fn parse_line_col(msg: &mut String) -> Option<(usize, usize)> {
    let start_of_suffix = match msg.rfind(" at line ") {
        Some(index) => index,
        None => return None,
    };
    let start_of_line = start_of_suffix + " at line ".len();
    let mut end_of_line = start_of_line;
    while starts_with_digit(&msg[end_of_line..]) {
        end_of_line += 1;
    }
    if !msg[end_of_line..].starts_with(" column ") {
        return None;
    }
    let start_of_column = end_of_line + " column ".len();
    let mut end_of_column = start_of_column;
    while starts_with_digit(&msg[end_of_column..]) {
        end_of_column += 1;
    }
    if end_of_column < msg.len() {
        return None;
    }
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
