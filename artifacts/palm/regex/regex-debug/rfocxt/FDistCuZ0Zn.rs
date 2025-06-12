type Result<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;
use std::error;
use std::io::{self, Write};
use std::process;
use std::result;
use docopt::Docopt;
use syntax::hir::Hir;
use syntax::hir::literal::Literals;
use regex::internal::{Compiler, LiteralSearcher};
const USAGE: &'static str = "
Usage:
    regex-debug [options] ast <pattern>
    regex-debug [options] hir <pattern>
    regex-debug [options] prefixes <patterns> ...
    regex-debug [options] suffixes <patterns> ...
    regex-debug [options] anchors <pattern>
    regex-debug [options] captures <pattern>
    regex-debug [options] compile <patterns> ...
    regex-debug [options] utf8-ranges <class>
    regex-debug --help

Options:
    --help               Show this usage message.
    --size-limit ARG     An approximate size limit on the total size (in bytes)
                         of a compiled regular expression program.
                         [default: 10485760]
    --bytes              Show the instruction codes for byte oriented programs.
                         (As opposed to Unicode oriented programs.)
    --dfa                Show the instruction codes for a DFA.
    --dfa-reverse        Show the instruction codes for a reverse DFA.
                         This implies --dfa.
    -a, --all-literals   Shows all literals extracted.
                         By default, only unambiguous literals are shown.
    --literal-limit ARG  An approximate limit on the total size (in bytes)
                         of all literals extracted. [default: 250]
    --class-limit ARG    A limit on the size of character classes used to
                         extract literals. [default: 10]
    --lcp                Show the longest common prefix of all the literals
                         extracted.
    --lcs                Show the longest common suffix of all the literals
                         extracted.
    --searcher           Show the debug output for the literal searcher
                         constructed by the literals found.
";
fn escape_unicode(bytes: &[u8]) -> String {
    let show = match ::std::str::from_utf8(bytes) {
        Ok(v) => v.to_string(),
        Err(_) => escape_bytes(bytes),
    };
    let mut space_escaped = String::new();
    for c in show.chars() {
        if c.is_whitespace() {
            let escaped = if c as u32 <= 0x7F {
                escape_byte(c as u8)
            } else {
                if c as u32 <= 0xFFFF {
                    format!(r"\u{{{:04x}}}", c as u32)
                } else {
                    format!(r"\U{{{:08x}}}", c as u32)
                }
            };
            space_escaped.push_str(&escaped);
        } else {
            space_escaped.push(c);
        }
    }
    space_escaped
}
fn escape_byte(byte: u8) -> String {
    use std::ascii::escape_default;
    let escaped: Vec<u8> = escape_default(byte).collect();
    String::from_utf8_lossy(&escaped).into_owned()
}
fn escape_bytes(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &b in bytes {
        s.push_str(&escape_byte(b));
    }
    s
}
