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
fn main() {
    let mut args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_dfa_reverse {
        args.flag_dfa = true;
    }
    match run(&args) {
        Ok(_) => process::exit(0),
        Err(err) => {
            let _ = writeln!(& mut io::stderr(), "{}", err);
            process::exit(1)
        }
    }
}
fn run(args: &Args) -> Result<()> {
    if args.cmd_ast {
        cmd_ast(args)
    } else if args.cmd_hir {
        cmd_hir(args)
    } else if args.cmd_prefixes {
        cmd_literals(args)
    } else if args.cmd_suffixes {
        cmd_literals(args)
    } else if args.cmd_anchors {
        cmd_anchors(args)
    } else if args.cmd_captures {
        cmd_captures(args)
    } else if args.cmd_compile {
        cmd_compile(args)
    } else if args.cmd_utf8_ranges {
        cmd_utf8_ranges(args)
    } else {
        unreachable!()
    }
}
