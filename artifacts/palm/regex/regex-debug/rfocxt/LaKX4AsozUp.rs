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
    ) -> Literals {}
    fn empty_literals(&self) -> Literals {}
    fn compiler(&self) -> Compiler {
        Compiler::new().size_limit(self.flag_size_limit)
    }
}
fn cmd_compile(args: &Args) -> Result<()> {
    let exprs = args.parse_many()?;
    let compiler = args
        .compiler()
        .bytes(args.flag_bytes)
        .only_utf8(!args.flag_bytes)
        .dfa(args.flag_dfa)
        .reverse(args.flag_dfa_reverse);
    let prog = compiler.compile(&exprs)?;
    print!("{:?}", prog);
    Ok(())
}
