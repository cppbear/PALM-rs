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
#[derive(Deserialize)]
struct Args {
    cmd_ast: bool,
    cmd_hir: bool,
    cmd_prefixes: bool,
    cmd_suffixes: bool,
    cmd_anchors: bool,
    cmd_captures: bool,
    cmd_compile: bool,
    cmd_utf8_ranges: bool,
    arg_pattern: String,
    arg_patterns: Vec<String>,
    arg_class: String,
    flag_size_limit: usize,
    flag_bytes: bool,
    flag_dfa: bool,
    flag_dfa_reverse: bool,
    flag_all_literals: bool,
    flag_literal_limit: usize,
    flag_class_limit: usize,
    flag_lcp: bool,
    flag_lcs: bool,
    flag_searcher: bool,
}
impl Args {
    fn parse_one(&self) -> Result<Hir> {}
    fn parse_many(&self) -> Result<Vec<Hir>> {
        self.arg_patterns.iter().map(|s| parse(s)).collect()
    }
    fn literals<F: Fn(&mut Literals, &Hir) -> bool>(
        &self,
        exprs: &[Hir],
        get_literals: F,
    ) -> Literals {
        let mut lits = Some(self.empty_literals());
        for e in exprs {
            lits = lits
                .and_then(|mut lits| {
                    if !get_literals(&mut lits, e) { None } else { Some(lits) }
                });
        }
        lits.unwrap_or(self.empty_literals())
    }
    fn empty_literals(&self) -> Literals {}
    fn compiler(&self) -> Compiler {}
}
fn cmd_literals(args: &Args) -> Result<()> {
    let exprs = args.parse_many()?;
    let mut lits = if args.cmd_prefixes {
        args.literals(&exprs, |lits, e| lits.union_prefixes(e))
    } else {
        args.literals(&exprs, |lits, e| lits.union_suffixes(e))
    };
    if !args.flag_all_literals {
        if args.cmd_prefixes {
            lits = lits.unambiguous_prefixes();
        } else {
            lits = lits.unambiguous_suffixes();
        }
    }
    if args.flag_searcher {
        if args.cmd_prefixes {
            println!("{:?}", LiteralSearcher::prefixes(lits))
        } else {
            println!("{:?}", LiteralSearcher::suffixes(lits))
        }
    } else if args.flag_lcp {
        println!("{}", escape_unicode(lits.longest_common_prefix()));
    } else if args.flag_lcs {
        println!("{}", escape_unicode(lits.longest_common_suffix()));
    } else {
        for lit in lits.literals() {
            println!("{:?}", lit);
        }
    }
    Ok(())
}
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
