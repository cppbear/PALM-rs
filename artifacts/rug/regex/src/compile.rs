use std::collections::HashMap;
use std::fmt;
use std::iter;
use std::result;
use std::sync::Arc;

use regex_syntax::hir::{self, Hir, Look};
use regex_syntax::is_word_byte;
use regex_syntax::utf8::{Utf8Range, Utf8Sequence, Utf8Sequences};

use crate::prog::{
    EmptyLook, Inst, InstBytes, InstChar, InstEmptyLook, InstPtr, InstRanges,
    InstSave, InstSplit, Program,
};

use crate::Error;

type Result = result::Result<Patch, Error>;
type ResultOrEmpty = result::Result<Option<Patch>, Error>;

#[derive(Debug)]
struct Patch {
    hole: Hole,
    entry: InstPtr,
}

/// A compiler translates a regular expression AST to a sequence of
/// instructions. The sequence of instructions represents an NFA.
// `Compiler` is only public via the `internal` module, so avoid deriving
// `Debug`.
#[allow(missing_debug_implementations)]
pub struct Compiler {
    insts: Vec<MaybeInst>,
    compiled: Program,
    capture_name_idx: HashMap<String, usize>,
    num_exprs: usize,
    size_limit: usize,
    suffix_cache: SuffixCache,
    utf8_seqs: Option<Utf8Sequences>,
    byte_classes: ByteClassSet,
    // This keeps track of extra bytes allocated while compiling the regex
    // program. Currently, this corresponds to two things. First is the heap
    // memory allocated by Unicode character classes ('InstRanges'). Second is
    // a "fake" amount of memory used by empty sub-expressions, so that enough
    // empty sub-expressions will ultimately trigger the compiler to bail
    // because of a size limit restriction. (That empty sub-expressions don't
    // add to heap memory usage is more-or-less an implementation detail.) In
    // the second case, if we don't bail, then an excessively large repetition
    // on an empty sub-expression can result in the compiler using a very large
    // amount of CPU time.
    extra_inst_bytes: usize,
}

impl Compiler {
    /// Create a new regular expression compiler.
    ///
    /// Various options can be set before calling `compile` on an expression.
    pub fn new() -> Self {
        Compiler {
            insts: vec![],
            compiled: Program::new(),
            capture_name_idx: HashMap::new(),
            num_exprs: 0,
            size_limit: 10 * (1 << 20),
            suffix_cache: SuffixCache::new(1000),
            utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
            byte_classes: ByteClassSet::new(),
            extra_inst_bytes: 0,
        }
    }

    /// The size of the resulting program is limited by size_limit. If
    /// the program approximately exceeds the given size (in bytes), then
    /// compilation will stop and return an error.
    pub fn size_limit(mut self, size_limit: usize) -> Self {
        self.size_limit = size_limit;
        self
    }

    /// If bytes is true, then the program is compiled as a byte based
    /// automaton, which incorporates UTF-8 decoding into the machine. If it's
    /// false, then the automaton is Unicode scalar value based, e.g., an
    /// engine utilizing such an automaton is responsible for UTF-8 decoding.
    ///
    /// The specific invariant is that when returning a byte based machine,
    /// the neither the `Char` nor `Ranges` instructions are produced.
    /// Conversely, when producing a Unicode scalar value machine, the `Bytes`
    /// instruction is never produced.
    ///
    /// Note that `dfa(true)` implies `bytes(true)`.
    pub fn bytes(mut self, yes: bool) -> Self {
        self.compiled.is_bytes = yes;
        self
    }

    /// When disabled, the program compiled may match arbitrary bytes.
    ///
    /// When enabled (the default), all compiled programs exclusively match
    /// valid UTF-8 bytes.
    pub fn only_utf8(mut self, yes: bool) -> Self {
        self.compiled.only_utf8 = yes;
        self
    }

    /// When set, the machine returned is suitable for use in the DFA matching
    /// engine.
    ///
    /// In particular, this ensures that if the regex is not anchored in the
    /// beginning, then a preceding `.*?` is included in the program. (The NFA
    /// based engines handle the preceding `.*?` explicitly, which is difficult
    /// or impossible in the DFA engine.)
    pub fn dfa(mut self, yes: bool) -> Self {
        self.compiled.is_dfa = yes;
        self
    }

    /// When set, the machine returned is suitable for matching text in
    /// reverse. In particular, all concatenations are flipped.
    pub fn reverse(mut self, yes: bool) -> Self {
        self.compiled.is_reverse = yes;
        self
    }

    /// Compile a regular expression given its AST.
    ///
    /// The compiler is guaranteed to succeed unless the program exceeds the
    /// specified size limit. If the size limit is exceeded, then compilation
    /// stops and returns an error.
    pub fn compile(mut self, exprs: &[Hir]) -> result::Result<Program, Error> {
        debug_assert!(!exprs.is_empty());
        self.num_exprs = exprs.len();
        if exprs.len() == 1 {
            self.compile_one(&exprs[0])
        } else {
            self.compile_many(exprs)
        }
    }

    fn compile_one(mut self, expr: &Hir) -> result::Result<Program, Error> {
        if self.compiled.only_utf8
            && expr.properties().look_set().contains(Look::WordAsciiNegate)
        {
            return Err(Error::Syntax(
                "ASCII-only \\B is not allowed in Unicode regexes \
                 because it may result in invalid UTF-8 matches"
                    .to_string(),
            ));
        }
        // If we're compiling a forward DFA and we aren't anchored, then
        // add a `.*?` before the first capture group.
        // Other matching engines handle this by baking the logic into the
        // matching engine itself.
        let mut dotstar_patch = Patch { hole: Hole::None, entry: 0 };
        self.compiled.is_anchored_start =
            expr.properties().look_set_prefix().contains(Look::Start);
        self.compiled.is_anchored_end =
            expr.properties().look_set_suffix().contains(Look::End);
        if self.compiled.needs_dotstar() {
            dotstar_patch = self.c_dotstar()?;
            self.compiled.start = dotstar_patch.entry;
        }
        self.compiled.captures = vec![None];
        let patch =
            self.c_capture(0, expr)?.unwrap_or_else(|| self.next_inst());
        if self.compiled.needs_dotstar() {
            self.fill(dotstar_patch.hole, patch.entry);
        } else {
            self.compiled.start = patch.entry;
        }
        self.fill_to_next(patch.hole);
        self.compiled.matches = vec![self.insts.len()];
        self.push_compiled(Inst::Match(0));
        self.compiled.static_captures_len =
            expr.properties().static_explicit_captures_len();
        self.compile_finish()
    }

    fn compile_many(
        mut self,
        exprs: &[Hir],
    ) -> result::Result<Program, Error> {
        debug_assert!(exprs.len() > 1);

        self.compiled.is_anchored_start = exprs
            .iter()
            .all(|e| e.properties().look_set_prefix().contains(Look::Start));
        self.compiled.is_anchored_end = exprs
            .iter()
            .all(|e| e.properties().look_set_suffix().contains(Look::End));
        let mut dotstar_patch = Patch { hole: Hole::None, entry: 0 };
        if self.compiled.needs_dotstar() {
            dotstar_patch = self.c_dotstar()?;
            self.compiled.start = dotstar_patch.entry;
        } else {
            self.compiled.start = 0; // first instruction is always split
        }
        self.fill_to_next(dotstar_patch.hole);

        let mut prev_hole = Hole::None;
        for (i, expr) in exprs[0..exprs.len() - 1].iter().enumerate() {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let Patch { hole, entry } =
                self.c_capture(0, expr)?.unwrap_or_else(|| self.next_inst());
            self.fill_to_next(hole);
            self.compiled.matches.push(self.insts.len());
            self.push_compiled(Inst::Match(i));
            prev_hole = self.fill_split(split, Some(entry), None);
        }
        let i = exprs.len() - 1;
        let Patch { hole, entry } =
            self.c_capture(0, &exprs[i])?.unwrap_or_else(|| self.next_inst());
        self.fill(prev_hole, entry);
        self.fill_to_next(hole);
        self.compiled.matches.push(self.insts.len());
        self.push_compiled(Inst::Match(i));
        self.compile_finish()
    }

    fn compile_finish(mut self) -> result::Result<Program, Error> {
        self.compiled.insts =
            self.insts.into_iter().map(|inst| inst.unwrap()).collect();
        self.compiled.byte_classes = self.byte_classes.byte_classes();
        self.compiled.capture_name_idx = Arc::new(self.capture_name_idx);
        Ok(self.compiled)
    }

    /// Compile expr into self.insts, returning a patch on success,
    /// or an error if we run out of memory.
    ///
    /// All of the c_* methods of the compiler share the contract outlined
    /// here.
    ///
    /// The main thing that a c_* method does is mutate `self.insts`
    /// to add a list of mostly compiled instructions required to execute
    /// the given expression. `self.insts` contains MaybeInsts rather than
    /// Insts because there is some backpatching required.
    ///
    /// The `Patch` value returned by each c_* method provides metadata
    /// about the compiled instructions emitted to `self.insts`. The
    /// `entry` member of the patch refers to the first instruction
    /// (the entry point), while the `hole` member contains zero or
    /// more offsets to partial instructions that need to be backpatched.
    /// The c_* routine can't know where its list of instructions are going to
    /// jump to after execution, so it is up to the caller to patch
    /// these jumps to point to the right place. So compiling some
    /// expression, e, we would end up with a situation that looked like:
    ///
    /// ```text
    /// self.insts = [ ..., i1, i2, ..., iexit1, ..., iexitn, ...]
    ///                     ^              ^             ^
    ///                     |                \         /
    ///                   entry                \     /
    ///                                         hole
    /// ```
    ///
    /// To compile two expressions, e1 and e2, concatenated together we
    /// would do:
    ///
    /// ```ignore
    /// let patch1 = self.c(e1);
    /// let patch2 = self.c(e2);
    /// ```
    ///
    /// while leaves us with a situation that looks like
    ///
    /// ```text
    /// self.insts = [ ..., i1, ..., iexit1, ..., i2, ..., iexit2 ]
    ///                     ^        ^            ^        ^
    ///                     |        |            |        |
    ///                entry1        hole1   entry2        hole2
    /// ```
    ///
    /// Then to merge the two patches together into one we would backpatch
    /// hole1 with entry2 and return a new patch that enters at entry1
    /// and has hole2 for a hole. In fact, if you look at the c_concat
    /// method you will see that it does exactly this, though it handles
    /// a list of expressions rather than just the two that we use for
    /// an example.
    ///
    /// Ok(None) is returned when an expression is compiled to no
    /// instruction, and so no patch.entry value makes sense.
    fn c(&mut self, expr: &Hir) -> ResultOrEmpty {
        use crate::prog;
        use regex_syntax::hir::HirKind::*;

        self.check_size()?;
        match *expr.kind() {
            Empty => self.c_empty(),
            Literal(hir::Literal(ref bytes)) => {
                if self.compiled.is_reverse {
                    let mut bytes = bytes.to_vec();
                    bytes.reverse();
                    self.c_literal(&bytes)
                } else {
                    self.c_literal(bytes)
                }
            }
            Class(hir::Class::Unicode(ref cls)) => self.c_class(cls.ranges()),
            Class(hir::Class::Bytes(ref cls)) => {
                if self.compiled.uses_bytes() {
                    self.c_class_bytes(cls.ranges())
                } else {
                    assert!(cls.is_ascii());
                    let mut char_ranges = vec![];
                    for r in cls.iter() {
                        let (s, e) = (r.start() as char, r.end() as char);
                        char_ranges.push(hir::ClassUnicodeRange::new(s, e));
                    }
                    self.c_class(&char_ranges)
                }
            }
            Look(ref look) => match *look {
                hir::Look::Start if self.compiled.is_reverse => {
                    self.c_empty_look(prog::EmptyLook::EndText)
                }
                hir::Look::Start => {
                    self.c_empty_look(prog::EmptyLook::StartText)
                }
                hir::Look::End if self.compiled.is_reverse => {
                    self.c_empty_look(prog::EmptyLook::StartText)
                }
                hir::Look::End => self.c_empty_look(prog::EmptyLook::EndText),
                hir::Look::StartLF if self.compiled.is_reverse => {
                    self.byte_classes.set_range(b'\n', b'\n');
                    self.c_empty_look(prog::EmptyLook::EndLine)
                }
                hir::Look::StartLF => {
                    self.byte_classes.set_range(b'\n', b'\n');
                    self.c_empty_look(prog::EmptyLook::StartLine)
                }
                hir::Look::EndLF if self.compiled.is_reverse => {
                    self.byte_classes.set_range(b'\n', b'\n');
                    self.c_empty_look(prog::EmptyLook::StartLine)
                }
                hir::Look::EndLF => {
                    self.byte_classes.set_range(b'\n', b'\n');
                    self.c_empty_look(prog::EmptyLook::EndLine)
                }
                hir::Look::StartCRLF | hir::Look::EndCRLF => {
                    return Err(Error::Syntax(
                        "CRLF-aware line anchors are not supported yet"
                            .to_string(),
                    ));
                }
                hir::Look::WordAscii => {
                    self.byte_classes.set_word_boundary();
                    self.c_empty_look(prog::EmptyLook::WordBoundaryAscii)
                }
                hir::Look::WordAsciiNegate => {
                    self.byte_classes.set_word_boundary();
                    self.c_empty_look(prog::EmptyLook::NotWordBoundaryAscii)
                }
                hir::Look::WordUnicode => {
                    if !cfg!(feature = "unicode-perl") {
                        return Err(Error::Syntax(
                            "Unicode word boundaries are unavailable when \
                         the unicode-perl feature is disabled"
                                .to_string(),
                        ));
                    }
                    self.compiled.has_unicode_word_boundary = true;
                    self.byte_classes.set_word_boundary();
                    // We also make sure that all ASCII bytes are in a different
                    // class from non-ASCII bytes. Otherwise, it's possible for
                    // ASCII bytes to get lumped into the same class as non-ASCII
                    // bytes. This in turn may cause the lazy DFA to falsely start
                    // when it sees an ASCII byte that maps to a byte class with
                    // non-ASCII bytes. This ensures that never happens.
                    self.byte_classes.set_range(0, 0x7F);
                    self.c_empty_look(prog::EmptyLook::WordBoundary)
                }
                hir::Look::WordUnicodeNegate => {
                    if !cfg!(feature = "unicode-perl") {
                        return Err(Error::Syntax(
                            "Unicode word boundaries are unavailable when \
                         the unicode-perl feature is disabled"
                                .to_string(),
                        ));
                    }
                    self.compiled.has_unicode_word_boundary = true;
                    self.byte_classes.set_word_boundary();
                    // See comments above for why we set the ASCII range here.
                    self.byte_classes.set_range(0, 0x7F);
                    self.c_empty_look(prog::EmptyLook::NotWordBoundary)
                }
            },
            Capture(hir::Capture { index, ref name, ref sub }) => {
                if index as usize >= self.compiled.captures.len() {
                    let name = match *name {
                        None => None,
                        Some(ref boxed_str) => Some(boxed_str.to_string()),
                    };
                    self.compiled.captures.push(name.clone());
                    if let Some(name) = name {
                        self.capture_name_idx.insert(name, index as usize);
                    }
                }
                self.c_capture(2 * index as usize, sub)
            }
            Concat(ref es) => {
                if self.compiled.is_reverse {
                    self.c_concat(es.iter().rev())
                } else {
                    self.c_concat(es)
                }
            }
            Alternation(ref es) => self.c_alternate(&**es),
            Repetition(ref rep) => self.c_repeat(rep),
        }
    }

    fn c_empty(&mut self) -> ResultOrEmpty {
        // See: https://github.com/rust-lang/regex/security/advisories/GHSA-m5pq-gvj9-9vr8
        // See: CVE-2022-24713
        //
        // Since 'empty' sub-expressions don't increase the size of
        // the actual compiled object, we "fake" an increase in its
        // size so that our 'check_size_limit' routine will eventually
        // stop compilation if there are too many empty sub-expressions
        // (e.g., via a large repetition).
        self.extra_inst_bytes += std::mem::size_of::<Inst>();
        Ok(None)
    }

    fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> ResultOrEmpty {
        if self.num_exprs > 1 || self.compiled.is_dfa {
            // Don't ever compile Save instructions for regex sets because
            // they are never used. They are also never used in DFA programs
            // because DFAs can't handle captures.
            self.c(expr)
        } else {
            let entry = self.insts.len();
            let hole = self.push_hole(InstHole::Save { slot: first_slot });
            let patch = self.c(expr)?.unwrap_or_else(|| self.next_inst());
            self.fill(hole, patch.entry);
            self.fill_to_next(patch.hole);
            let hole = self.push_hole(InstHole::Save { slot: first_slot + 1 });
            Ok(Some(Patch { hole, entry }))
        }
    }

    fn c_dotstar(&mut self) -> Result {
        let hir = if self.compiled.only_utf8() {
            Hir::dot(hir::Dot::AnyChar)
        } else {
            Hir::dot(hir::Dot::AnyByte)
        };
        Ok(self
            .c(&Hir::repetition(hir::Repetition {
                min: 0,
                max: None,
                greedy: false,
                sub: Box::new(hir),
            }))?
            .unwrap())
    }

    fn c_char(&mut self, c: char) -> ResultOrEmpty {
        if self.compiled.uses_bytes() {
            if c.is_ascii() {
                let b = c as u8;
                let hole =
                    self.push_hole(InstHole::Bytes { start: b, end: b });
                self.byte_classes.set_range(b, b);
                Ok(Some(Patch { hole, entry: self.insts.len() - 1 }))
            } else {
                self.c_class(&[hir::ClassUnicodeRange::new(c, c)])
            }
        } else {
            let hole = self.push_hole(InstHole::Char { c });
            Ok(Some(Patch { hole, entry: self.insts.len() - 1 }))
        }
    }

    fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> ResultOrEmpty {
        use std::mem::size_of;

        if ranges.is_empty() {
            return Err(Error::Syntax(
                "empty character classes are not allowed".to_string(),
            ));
        }
        if self.compiled.uses_bytes() {
            Ok(Some(CompileClass { c: self, ranges }.compile()?))
        } else {
            let ranges: Vec<(char, char)> =
                ranges.iter().map(|r| (r.start(), r.end())).collect();
            let hole = if ranges.len() == 1 && ranges[0].0 == ranges[0].1 {
                self.push_hole(InstHole::Char { c: ranges[0].0 })
            } else {
                self.extra_inst_bytes +=
                    ranges.len() * (size_of::<char>() * 2);
                self.push_hole(InstHole::Ranges { ranges })
            };
            Ok(Some(Patch { hole, entry: self.insts.len() - 1 }))
        }
    }

    fn c_byte(&mut self, b: u8) -> ResultOrEmpty {
        self.c_class_bytes(&[hir::ClassBytesRange::new(b, b)])
    }

    fn c_class_bytes(
        &mut self,
        ranges: &[hir::ClassBytesRange],
    ) -> ResultOrEmpty {
        if ranges.is_empty() {
            return Err(Error::Syntax(
                "empty character classes are not allowed".to_string(),
            ));
        }

        let first_split_entry = self.insts.len();
        let mut holes = vec![];
        let mut prev_hole = Hole::None;
        for r in &ranges[0..ranges.len() - 1] {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let next = self.insts.len();
            self.byte_classes.set_range(r.start(), r.end());
            holes.push(self.push_hole(InstHole::Bytes {
                start: r.start(),
                end: r.end(),
            }));
            prev_hole = self.fill_split(split, Some(next), None);
        }
        let next = self.insts.len();
        let r = &ranges[ranges.len() - 1];
        self.byte_classes.set_range(r.start(), r.end());
        holes.push(
            self.push_hole(InstHole::Bytes { start: r.start(), end: r.end() }),
        );
        self.fill(prev_hole, next);
        Ok(Some(Patch { hole: Hole::Many(holes), entry: first_split_entry }))
    }

    fn c_empty_look(&mut self, look: EmptyLook) -> ResultOrEmpty {
        let hole = self.push_hole(InstHole::EmptyLook { look });
        Ok(Some(Patch { hole, entry: self.insts.len() - 1 }))
    }

    fn c_literal(&mut self, bytes: &[u8]) -> ResultOrEmpty {
        match core::str::from_utf8(bytes) {
            Ok(string) => {
                let mut it = string.chars();
                let Patch { mut hole, entry } = loop {
                    match it.next() {
                        None => return self.c_empty(),
                        Some(ch) => {
                            if let Some(p) = self.c_char(ch)? {
                                break p;
                            }
                        }
                    }
                };
                for ch in it {
                    if let Some(p) = self.c_char(ch)? {
                        self.fill(hole, p.entry);
                        hole = p.hole;
                    }
                }
                Ok(Some(Patch { hole, entry }))
            }
            Err(_) => {
                assert!(self.compiled.uses_bytes());
                let mut it = bytes.iter().copied();
                let Patch { mut hole, entry } = loop {
                    match it.next() {
                        None => return self.c_empty(),
                        Some(byte) => {
                            if let Some(p) = self.c_byte(byte)? {
                                break p;
                            }
                        }
                    }
                };
                for byte in it {
                    if let Some(p) = self.c_byte(byte)? {
                        self.fill(hole, p.entry);
                        hole = p.hole;
                    }
                }
                Ok(Some(Patch { hole, entry }))
            }
        }
    }

    fn c_concat<'a, I>(&mut self, exprs: I) -> ResultOrEmpty
    where
        I: IntoIterator<Item = &'a Hir>,
    {
        let mut exprs = exprs.into_iter();
        let Patch { mut hole, entry } = loop {
            match exprs.next() {
                None => return self.c_empty(),
                Some(e) => {
                    if let Some(p) = self.c(e)? {
                        break p;
                    }
                }
            }
        };
        for e in exprs {
            if let Some(p) = self.c(e)? {
                self.fill(hole, p.entry);
                hole = p.hole;
            }
        }
        Ok(Some(Patch { hole, entry }))
    }

    fn c_alternate(&mut self, exprs: &[Hir]) -> ResultOrEmpty {
        debug_assert!(
            exprs.len() >= 2,
            "alternates must have at least 2 exprs"
        );

        // Initial entry point is always the first split.
        let first_split_entry = self.insts.len();

        // Save up all of the holes from each alternate. They will all get
        // patched to point to the same location.
        let mut holes = vec![];

        // true indicates that the hole is a split where we want to fill
        // the second branch.
        let mut prev_hole = (Hole::None, false);
        for e in &exprs[0..exprs.len() - 1] {
            if prev_hole.1 {
                let next = self.insts.len();
                self.fill_split(prev_hole.0, None, Some(next));
            } else {
                self.fill_to_next(prev_hole.0);
            }
            let split = self.push_split_hole();
            if let Some(Patch { hole, entry }) = self.c(e)? {
                holes.push(hole);
                prev_hole = (self.fill_split(split, Some(entry), None), false);
            } else {
                let (split1, split2) = split.dup_one();
                holes.push(split1);
                prev_hole = (split2, true);
            }
        }
        if let Some(Patch { hole, entry }) = self.c(&exprs[exprs.len() - 1])? {
            holes.push(hole);
            if prev_hole.1 {
                self.fill_split(prev_hole.0, None, Some(entry));
            } else {
                self.fill(prev_hole.0, entry);
            }
        } else {
            // We ignore prev_hole.1. When it's true, it means we have two
            // empty branches both pushing prev_hole.0 into holes, so both
            // branches will go to the same place anyway.
            holes.push(prev_hole.0);
        }
        Ok(Some(Patch { hole: Hole::Many(holes), entry: first_split_entry }))
    }

    fn c_repeat(&mut self, rep: &hir::Repetition) -> ResultOrEmpty {
        match (rep.min, rep.max) {
            (0, Some(1)) => self.c_repeat_zero_or_one(&rep.sub, rep.greedy),
            (0, None) => self.c_repeat_zero_or_more(&rep.sub, rep.greedy),
            (1, None) => self.c_repeat_one_or_more(&rep.sub, rep.greedy),
            (min, None) => {
                self.c_repeat_range_min_or_more(&rep.sub, rep.greedy, min)
            }
            (min, Some(max)) => {
                self.c_repeat_range(&rep.sub, rep.greedy, min, max)
            }
        }
    }

    fn c_repeat_zero_or_one(
        &mut self,
        expr: &Hir,
        greedy: bool,
    ) -> ResultOrEmpty {
        let split_entry = self.insts.len();
        let split = self.push_split_hole();
        let Patch { hole: hole_rep, entry: entry_rep } = match self.c(expr)? {
            Some(p) => p,
            None => return self.pop_split_hole(),
        };
        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        let holes = vec![hole_rep, split_hole];
        Ok(Some(Patch { hole: Hole::Many(holes), entry: split_entry }))
    }

    fn c_repeat_zero_or_more(
        &mut self,
        expr: &Hir,
        greedy: bool,
    ) -> ResultOrEmpty {
        let split_entry = self.insts.len();
        let split = self.push_split_hole();
        let Patch { hole: hole_rep, entry: entry_rep } = match self.c(expr)? {
            Some(p) => p,
            None => return self.pop_split_hole(),
        };

        self.fill(hole_rep, split_entry);
        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        Ok(Some(Patch { hole: split_hole, entry: split_entry }))
    }

    fn c_repeat_one_or_more(
        &mut self,
        expr: &Hir,
        greedy: bool,
    ) -> ResultOrEmpty {
        let Patch { hole: hole_rep, entry: entry_rep } = match self.c(expr)? {
            Some(p) => p,
            None => return Ok(None),
        };
        self.fill_to_next(hole_rep);
        let split = self.push_split_hole();

        let split_hole = if greedy {
            self.fill_split(split, Some(entry_rep), None)
        } else {
            self.fill_split(split, None, Some(entry_rep))
        };
        Ok(Some(Patch { hole: split_hole, entry: entry_rep }))
    }

    fn c_repeat_range_min_or_more(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
    ) -> ResultOrEmpty {
        let min = u32_to_usize(min);
        // Using next_inst() is ok, because we can't return it (concat would
        // have to return Some(_) while c_repeat_range_min_or_more returns
        // None).
        let patch_concat = self
            .c_concat(iter::repeat(expr).take(min))?
            .unwrap_or_else(|| self.next_inst());
        if let Some(patch_rep) = self.c_repeat_zero_or_more(expr, greedy)? {
            self.fill(patch_concat.hole, patch_rep.entry);
            Ok(Some(Patch { hole: patch_rep.hole, entry: patch_concat.entry }))
        } else {
            Ok(None)
        }
    }

    fn c_repeat_range(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
        max: u32,
    ) -> ResultOrEmpty {
        let (min, max) = (u32_to_usize(min), u32_to_usize(max));
        debug_assert!(min <= max);
        let patch_concat = self.c_concat(iter::repeat(expr).take(min))?;
        if min == max {
            return Ok(patch_concat);
        }
        // Same reasoning as in c_repeat_range_min_or_more (we know that min <
        // max at this point).
        let patch_concat = patch_concat.unwrap_or_else(|| self.next_inst());
        let initial_entry = patch_concat.entry;
        // It is much simpler to compile, e.g., `a{2,5}` as:
        //
        //     aaa?a?a?
        //
        // But you end up with a sequence of instructions like this:
        //
        //     0: 'a'
        //     1: 'a',
        //     2: split(3, 4)
        //     3: 'a'
        //     4: split(5, 6)
        //     5: 'a'
        //     6: split(7, 8)
        //     7: 'a'
        //     8: MATCH
        //
        // This is *incredibly* inefficient because the splits end
        // up forming a chain, which has to be resolved everything a
        // transition is followed.
        let mut holes = vec![];
        let mut prev_hole = patch_concat.hole;
        for _ in min..max {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let Patch { hole, entry } = match self.c(expr)? {
                Some(p) => p,
                None => return self.pop_split_hole(),
            };
            prev_hole = hole;
            if greedy {
                holes.push(self.fill_split(split, Some(entry), None));
            } else {
                holes.push(self.fill_split(split, None, Some(entry)));
            }
        }
        holes.push(prev_hole);
        Ok(Some(Patch { hole: Hole::Many(holes), entry: initial_entry }))
    }

    /// Can be used as a default value for the c_* functions when the call to
    /// c_function is followed by inserting at least one instruction that is
    /// always executed after the ones written by the c* function.
    fn next_inst(&self) -> Patch {
        Patch { hole: Hole::None, entry: self.insts.len() }
    }

    fn fill(&mut self, hole: Hole, goto: InstPtr) {
        match hole {
            Hole::None => {}
            Hole::One(pc) => {
                self.insts[pc].fill(goto);
            }
            Hole::Many(holes) => {
                for hole in holes {
                    self.fill(hole, goto);
                }
            }
        }
    }

    fn fill_to_next(&mut self, hole: Hole) {
        let next = self.insts.len();
        self.fill(hole, next);
    }

    fn fill_split(
        &mut self,
        hole: Hole,
        goto1: Option<InstPtr>,
        goto2: Option<InstPtr>,
    ) -> Hole {
        match hole {
            Hole::None => Hole::None,
            Hole::One(pc) => match (goto1, goto2) {
                (Some(goto1), Some(goto2)) => {
                    self.insts[pc].fill_split(goto1, goto2);
                    Hole::None
                }
                (Some(goto1), None) => {
                    self.insts[pc].half_fill_split_goto1(goto1);
                    Hole::One(pc)
                }
                (None, Some(goto2)) => {
                    self.insts[pc].half_fill_split_goto2(goto2);
                    Hole::One(pc)
                }
                (None, None) => unreachable!(
                    "at least one of the split \
                     holes must be filled"
                ),
            },
            Hole::Many(holes) => {
                let mut new_holes = vec![];
                for hole in holes {
                    new_holes.push(self.fill_split(hole, goto1, goto2));
                }
                if new_holes.is_empty() {
                    Hole::None
                } else if new_holes.len() == 1 {
                    new_holes.pop().unwrap()
                } else {
                    Hole::Many(new_holes)
                }
            }
        }
    }

    fn push_compiled(&mut self, inst: Inst) {
        self.insts.push(MaybeInst::Compiled(inst));
    }

    fn push_hole(&mut self, inst: InstHole) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst::Uncompiled(inst));
        Hole::One(hole)
    }

    fn push_split_hole(&mut self) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst::Split);
        Hole::One(hole)
    }

    fn pop_split_hole(&mut self) -> ResultOrEmpty {
        self.insts.pop();
        Ok(None)
    }

    fn check_size(&self) -> result::Result<(), Error> {
        use std::mem::size_of;

        let size =
            self.extra_inst_bytes + (self.insts.len() * size_of::<Inst>());
        if size > self.size_limit {
            Err(Error::CompiledTooBig(self.size_limit))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
enum Hole {
    None,
    One(InstPtr),
    Many(Vec<Hole>),
}

impl Hole {
    fn dup_one(self) -> (Self, Self) {
        match self {
            Hole::One(pc) => (Hole::One(pc), Hole::One(pc)),
            Hole::None | Hole::Many(_) => {
                unreachable!("must be called on single hole")
            }
        }
    }
}

#[derive(Clone, Debug)]
enum MaybeInst {
    Compiled(Inst),
    Uncompiled(InstHole),
    Split,
    Split1(InstPtr),
    Split2(InstPtr),
}

impl MaybeInst {
    fn fill(&mut self, goto: InstPtr) {
        let maybeinst = match *self {
            MaybeInst::Split => MaybeInst::Split1(goto),
            MaybeInst::Uncompiled(ref inst) => {
                MaybeInst::Compiled(inst.fill(goto))
            }
            MaybeInst::Split1(goto1) => {
                MaybeInst::Compiled(Inst::Split(InstSplit {
                    goto1,
                    goto2: goto,
                }))
            }
            MaybeInst::Split2(goto2) => {
                MaybeInst::Compiled(Inst::Split(InstSplit {
                    goto1: goto,
                    goto2,
                }))
            }
            _ => unreachable!(
                "not all instructions were compiled! \
                 found uncompiled instruction: {:?}",
                self
            ),
        };
        *self = maybeinst;
    }

    fn fill_split(&mut self, goto1: InstPtr, goto2: InstPtr) {
        let filled = match *self {
            MaybeInst::Split => Inst::Split(InstSplit { goto1, goto2 }),
            _ => unreachable!(
                "must be called on Split instruction, \
                 instead it was called on: {:?}",
                self
            ),
        };
        *self = MaybeInst::Compiled(filled);
    }

    fn half_fill_split_goto1(&mut self, goto1: InstPtr) {
        let half_filled = match *self {
            MaybeInst::Split => goto1,
            _ => unreachable!(
                "must be called on Split instruction, \
                 instead it was called on: {:?}",
                self
            ),
        };
        *self = MaybeInst::Split1(half_filled);
    }

    fn half_fill_split_goto2(&mut self, goto2: InstPtr) {
        let half_filled = match *self {
            MaybeInst::Split => goto2,
            _ => unreachable!(
                "must be called on Split instruction, \
                 instead it was called on: {:?}",
                self
            ),
        };
        *self = MaybeInst::Split2(half_filled);
    }

    fn unwrap(self) -> Inst {
        match self {
            MaybeInst::Compiled(inst) => inst,
            _ => unreachable!(
                "must be called on a compiled instruction, \
                 instead it was called on: {:?}",
                self
            ),
        }
    }
}

#[derive(Clone, Debug)]
enum InstHole {
    Save { slot: usize },
    EmptyLook { look: EmptyLook },
    Char { c: char },
    Ranges { ranges: Vec<(char, char)> },
    Bytes { start: u8, end: u8 },
}

impl InstHole {
    fn fill(&self, goto: InstPtr) -> Inst {
        match *self {
            InstHole::Save { slot } => Inst::Save(InstSave { goto, slot }),
            InstHole::EmptyLook { look } => {
                Inst::EmptyLook(InstEmptyLook { goto, look })
            }
            InstHole::Char { c } => Inst::Char(InstChar { goto, c }),
            InstHole::Ranges { ref ranges } => Inst::Ranges(InstRanges {
                goto,
                ranges: ranges.clone().into_boxed_slice(),
            }),
            InstHole::Bytes { start, end } => {
                Inst::Bytes(InstBytes { goto, start, end })
            }
        }
    }
}

struct CompileClass<'a, 'b> {
    c: &'a mut Compiler,
    ranges: &'b [hir::ClassUnicodeRange],
}

impl<'a, 'b> CompileClass<'a, 'b> {
    fn compile(mut self) -> Result {
        let mut holes = vec![];
        let mut initial_entry = None;
        let mut last_split = Hole::None;
        let mut utf8_seqs = self.c.utf8_seqs.take().unwrap();
        self.c.suffix_cache.clear();

        for (i, range) in self.ranges.iter().enumerate() {
            let is_last_range = i + 1 == self.ranges.len();
            utf8_seqs.reset(range.start(), range.end());
            let mut it = (&mut utf8_seqs).peekable();
            loop {
                let utf8_seq = match it.next() {
                    None => break,
                    Some(utf8_seq) => utf8_seq,
                };
                if is_last_range && it.peek().is_none() {
                    let Patch { hole, entry } = self.c_utf8_seq(&utf8_seq)?;
                    holes.push(hole);
                    self.c.fill(last_split, entry);
                    last_split = Hole::None;
                    if initial_entry.is_none() {
                        initial_entry = Some(entry);
                    }
                } else {
                    if initial_entry.is_none() {
                        initial_entry = Some(self.c.insts.len());
                    }
                    self.c.fill_to_next(last_split);
                    last_split = self.c.push_split_hole();
                    let Patch { hole, entry } = self.c_utf8_seq(&utf8_seq)?;
                    holes.push(hole);
                    last_split =
                        self.c.fill_split(last_split, Some(entry), None);
                }
            }
        }
        self.c.utf8_seqs = Some(utf8_seqs);
        Ok(Patch { hole: Hole::Many(holes), entry: initial_entry.unwrap() })
    }

    fn c_utf8_seq(&mut self, seq: &Utf8Sequence) -> Result {
        if self.c.compiled.is_reverse {
            self.c_utf8_seq_(seq)
        } else {
            self.c_utf8_seq_(seq.into_iter().rev())
        }
    }

    fn c_utf8_seq_<'r, I>(&mut self, seq: I) -> Result
    where
        I: IntoIterator<Item = &'r Utf8Range>,
    {
        // The initial instruction for each UTF-8 sequence should be the same.
        let mut from_inst = ::std::usize::MAX;
        let mut last_hole = Hole::None;
        for byte_range in seq {
            let key = SuffixCacheKey {
                from_inst,
                start: byte_range.start,
                end: byte_range.end,
            };
            {
                let pc = self.c.insts.len();
                if let Some(cached_pc) = self.c.suffix_cache.get(key, pc) {
                    from_inst = cached_pc;
                    continue;
                }
            }
            self.c.byte_classes.set_range(byte_range.start, byte_range.end);
            if from_inst == ::std::usize::MAX {
                last_hole = self.c.push_hole(InstHole::Bytes {
                    start: byte_range.start,
                    end: byte_range.end,
                });
            } else {
                self.c.push_compiled(Inst::Bytes(InstBytes {
                    goto: from_inst,
                    start: byte_range.start,
                    end: byte_range.end,
                }));
            }
            from_inst = self.c.insts.len().checked_sub(1).unwrap();
            debug_assert!(from_inst < ::std::usize::MAX);
        }
        debug_assert!(from_inst < ::std::usize::MAX);
        Ok(Patch { hole: last_hole, entry: from_inst })
    }
}

/// `SuffixCache` is a simple bounded hash map for caching suffix entries in
/// UTF-8 automata. For example, consider the Unicode range \u{0}-\u{FFFF}.
/// The set of byte ranges looks like this:
///
/// [0-7F]
/// [C2-DF][80-BF]
/// [E0][A0-BF][80-BF]
/// [E1-EC][80-BF][80-BF]
/// [ED][80-9F][80-BF]
/// [EE-EF][80-BF][80-BF]
///
/// Each line above translates to one alternate in the compiled regex program.
/// However, all but one of the alternates end in the same suffix, which is
/// a waste of an instruction. The suffix cache facilitates reusing them across
/// alternates.
///
/// Note that a HashMap could be trivially used for this, but we don't need its
/// overhead. Some small bounded space (LRU style) is more than enough.
///
/// This uses similar idea to [`SparseSet`](../sparse/struct.SparseSet.html),
/// except it uses hashes as original indices and then compares full keys for
/// validation against `dense` array.
#[derive(Debug)]
struct SuffixCache {
    sparse: Box<[usize]>,
    dense: Vec<SuffixCacheEntry>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct SuffixCacheEntry {
    key: SuffixCacheKey,
    pc: InstPtr,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct SuffixCacheKey {
    from_inst: InstPtr,
    start: u8,
    end: u8,
}

impl SuffixCache {
    fn new(size: usize) -> Self {
        SuffixCache {
            sparse: vec![0usize; size].into(),
            dense: Vec::with_capacity(size),
        }
    }

    fn get(&mut self, key: SuffixCacheKey, pc: InstPtr) -> Option<InstPtr> {
        let hash = self.hash(&key);
        let pos = &mut self.sparse[hash];
        if let Some(entry) = self.dense.get(*pos) {
            if entry.key == key {
                return Some(entry.pc);
            }
        }
        *pos = self.dense.len();
        self.dense.push(SuffixCacheEntry { key, pc });
        None
    }

    fn clear(&mut self) {
        self.dense.clear();
    }

    fn hash(&self, suffix: &SuffixCacheKey) -> usize {
        // Basic FNV-1a hash as described:
        // https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
        const FNV_PRIME: u64 = 1_099_511_628_211;
        let mut h = 14_695_981_039_346_656_037;
        h = (h ^ (suffix.from_inst as u64)).wrapping_mul(FNV_PRIME);
        h = (h ^ (suffix.start as u64)).wrapping_mul(FNV_PRIME);
        h = (h ^ (suffix.end as u64)).wrapping_mul(FNV_PRIME);
        (h as usize) % self.sparse.len()
    }
}

struct ByteClassSet([bool; 256]);

impl ByteClassSet {
    fn new() -> Self {
        ByteClassSet([false; 256])
    }

    fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0[start as usize - 1] = true;
        }
        self.0[end as usize] = true;
    }

    fn set_word_boundary(&mut self) {
        // We need to mark all ranges of bytes whose pairs result in
        // evaluating \b differently.
        let iswb = is_word_byte;
        let mut b1: u16 = 0;
        let mut b2: u16;
        while b1 <= 255 {
            b2 = b1 + 1;
            while b2 <= 255 && iswb(b1 as u8) == iswb(b2 as u8) {
                b2 += 1;
            }
            self.set_range(b1 as u8, (b2 - 1) as u8);
            b1 = b2;
        }
    }

    fn byte_classes(&self) -> Vec<u8> {
        // N.B. If you're debugging the DFA, it's useful to simply return
        // `(0..256).collect()`, which effectively removes the byte classes
        // and makes the transitions easier to read.
        // (0usize..256).map(|x| x as u8).collect()
        let mut byte_classes = vec![0; 256];
        let mut class = 0u8;
        let mut i = 0;
        loop {
            byte_classes[i] = class as u8;
            if i >= 255 {
                break;
            }
            if self.0[i] {
                class = class.checked_add(1).unwrap();
            }
            i += 1;
        }
        byte_classes
    }
}

impl fmt::Debug for ByteClassSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ByteClassSet").field(&&self.0[..]).finish()
    }
}

fn u32_to_usize(n: u32) -> usize {
    // In case usize is less than 32 bits, we need to guard against overflow.
    // On most platforms this compiles to nothing.
    // TODO Use `std::convert::TryFrom` once it's stable.
    if (n as u64) > (::std::usize::MAX as u64) {
        panic!("BUG: {} is too big to be pointer sized", n)
    }
    n as usize
}

#[cfg(test)]
mod tests {
    use super::ByteClassSet;

    #[test]
    fn byte_classes() {
        let mut set = ByteClassSet::new();
        set.set_range(b'a', b'z');
        let classes = set.byte_classes();
        assert_eq!(classes[0], 0);
        assert_eq!(classes[1], 0);
        assert_eq!(classes[2], 0);
        assert_eq!(classes[b'a' as usize - 1], 0);
        assert_eq!(classes[b'a' as usize], 1);
        assert_eq!(classes[b'm' as usize], 1);
        assert_eq!(classes[b'z' as usize], 1);
        assert_eq!(classes[b'z' as usize + 1], 2);
        assert_eq!(classes[254], 2);
        assert_eq!(classes[255], 2);

        let mut set = ByteClassSet::new();
        set.set_range(0, 2);
        set.set_range(4, 6);
        let classes = set.byte_classes();
        assert_eq!(classes[0], 0);
        assert_eq!(classes[1], 0);
        assert_eq!(classes[2], 0);
        assert_eq!(classes[3], 1);
        assert_eq!(classes[4], 2);
        assert_eq!(classes[5], 2);
        assert_eq!(classes[6], 2);
        assert_eq!(classes[7], 3);
        assert_eq!(classes[255], 3);
    }

    #[test]
    fn full_byte_classes() {
        let mut set = ByteClassSet::new();
        for i in 0..256u16 {
            set.set_range(i as u8, i as u8);
        }
        assert_eq!(set.byte_classes().len(), 256);
    }
}

#[cfg(test)]
mod tests_llm_16_221 {
    use super::*;

use crate::*;

    #[test]
    fn test_byte_class_set_new() {
        let bcs = ByteClassSet::new();
        assert_eq!(bcs.0, [false; 256]);
    }
}

#[cfg(test)]
mod tests_llm_16_222 {
    use super::*;

use crate::*;

    #[test]
    fn test_set_range() {
        let mut bcs = ByteClassSet::new();

        // Test the case where start = end
        bcs.set_range(5, 5);
        assert!(bcs.0[4]); // start index should be set
        assert!(bcs.0[5]); // end index should be set

        // Test a range starting from 0 to 5
        bcs.set_range(0, 5);
        assert!(bcs.0[0]); // start index
        assert!(bcs.0[5]); // end index

        // Test a wider range
        bcs.set_range(10, 15);
        assert!(bcs.0[9]);  // start index of second range
        assert!(bcs.0[15]); // end index

        // Verify unmodified indices
        assert!(!bcs.0[6]); // index 6 should not be set
        assert!(!bcs.0[9]); // index 9 should not be set before the above range
        
        // Test the upper limit
        bcs.set_range(250, 255);
        assert!(bcs.0[249]); // start index
        assert!(bcs.0[255]); // end index

        // Verify indices outside range
        assert!(!bcs.0[256]); // index 256 should not be set (out of bounds)
    }
}

#[cfg(test)]
mod tests_llm_16_226 {
    use super::*;

use crate::*;
    use crate::compile::{Compiler, CompileClass};
    use regex_syntax::hir::{Hir, Class, ClassUnicodeRange};

    #[test]
    fn test_compile_empty_range() {
        let mut compiler = Compiler::new();
        let ranges: Vec<ClassUnicodeRange> = vec![];
        let compile_class = CompileClass {
            c: &mut compiler,
            ranges: &ranges,
        };
        let result = compile_class.compile();
        assert!(result.is_err());
    }

    #[test]
    fn test_compile_single_character() {
        let mut compiler = Compiler::new();
        let ranges: Vec<ClassUnicodeRange> = vec![ClassUnicodeRange::new('a', 'a')];
        let compile_class = CompileClass {
            c: &mut compiler,
            ranges: &ranges,
        };
        let result = compile_class.compile().unwrap();
        assert_eq!(result.entry, 0);
        assert_eq!(compiler.insts.len(), 1);
    }

    #[test]
    fn test_compile_multiple_characters() {
        let mut compiler = Compiler::new();
        let ranges: Vec<ClassUnicodeRange> = vec![
            ClassUnicodeRange::new('a', 'b'),
            ClassUnicodeRange::new('d', 'e'),
        ];
        let compile_class = CompileClass {
            c: &mut compiler,
            ranges: &ranges,
        };
        let result = compile_class.compile().unwrap();
        assert_eq!(result.entry, 0);
        assert!(compiler.insts.len() > 1); // Expect multiple instructions for ranges
    }

    #[test]
    fn test_compile_unicode_range() {
        let mut compiler = Compiler::new();
        let ranges: Vec<ClassUnicodeRange> = vec![ClassUnicodeRange::new('α', 'ω')];
        let compile_class = CompileClass {
            c: &mut compiler,
            ranges: &ranges,
        };
        let result = compile_class.compile().unwrap();
        assert_eq!(result.entry, 0);
        assert!(compiler.insts.len() > 0); // Expect at least one instruction
    }

    #[test]
    fn test_compile_filled_holes() {
        let mut compiler = Compiler::new();
        let ranges: Vec<ClassUnicodeRange> = vec![ClassUnicodeRange::new('x', 'y')];
        let compile_class = CompileClass {
            c: &mut compiler,
            ranges: &ranges,
        };
        let result = compile_class.compile().unwrap();
        let patch = result.hole;
        assert!(matches!(patch, Hole::Many(_))); // Expect filled holes
    }
}

#[cfg(test)]
mod tests_llm_16_230 {
    use super::*;

use crate::*;
    use crate::compile::Compiler;

    #[test]
    fn test_c_byte() {
        let mut compiler = Compiler::new();
        
        // Testing valid byte
        assert!(compiler.c_byte(b'a').is_ok());
        
        // Testing another valid byte
        assert!(compiler.c_byte(b'\n').is_ok());
        
        // Testing boundary byte
        assert!(compiler.c_byte(0).is_ok());
        assert!(compiler.c_byte(255).is_ok());
        
        // Testing invalid byte (the function should always accept u8)
        // Since u8 is always valid, we can only test valid ranges
    }
}

#[cfg(test)]
mod tests_llm_16_233 {
    use super::*;

use crate::*;
    use regex_syntax::hir::ClassUnicodeRange;

    #[test]
    fn test_c_class_empty_ranges() {
        let mut compiler = Compiler::new();
        let result = compiler.c_class(&[]);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "empty character classes are not allowed");
    }

    #[test]
    fn test_c_class_single_char() {
        let mut compiler = Compiler::new();
        let ranges = vec![ClassUnicodeRange::new('a', 'a')]; // Test with a single character
        let result = compiler.c_class(&ranges);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some()); // Ensure a patch is returned
    }

    #[test]
    fn test_c_class_multiple_chars() {
        let mut compiler = Compiler::new();
        let ranges = vec![ClassUnicodeRange::new('a', 'b'), ClassUnicodeRange::new('d', 'd')]; // Test with multiple ranges
        let result = compiler.c_class(&ranges);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some()); // Ensure a patch is returned
    }

    #[test]
    fn test_c_class_ranges() {
        let mut compiler = Compiler::new();
        let ranges = vec![ClassUnicodeRange::new('c', 'f')]; // Test with a character range
        let result = compiler.c_class(&ranges);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some()); // Ensure a patch is returned
    }

    #[test]
    fn test_c_class_bytes() {
        let mut compiler = Compiler::new().bytes(true); // Test byte mode
        let ranges = vec![ClassUnicodeRange::new('A', 'Z')]; // ASCII range
        let result = compiler.c_class(&ranges);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some()); // Ensure a patch is returned
    }
}

#[cfg(test)]
mod tests_llm_16_238 {
    use super::*;

use crate::*;
    use crate::compile::{Compiler, ByteClassSet};
    use crate::prog::EmptyLook;

    #[test]
    fn test_c_empty_look_start_text() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::StartText);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_end_text() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::EndText);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_start_line() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::StartLine);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_end_line() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::EndLine);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_word_boundary() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::WordBoundary);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_not_word_boundary() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::NotWordBoundary);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_word_boundary_ascii() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::WordBoundaryAscii);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }

    #[test]
    fn test_c_empty_look_not_word_boundary_ascii() {
        let mut compiler = Compiler::new();
        let result = compiler.c_empty_look(EmptyLook::NotWordBoundaryAscii);
        assert!(result.is_ok());
        let patch = result.unwrap().unwrap();
        assert!(patch.entry == compiler.insts.len() - 1);
    }
}

#[cfg(test)]
mod tests_llm_16_243 {
    use super::*;

use crate::*;
    use regex_syntax::hir::{Hir, HirKind};
    use regex_syntax::hir::Repetition;

    #[test]
    fn test_c_repeat_range_min_or_more_greedy() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 2,
            max: None,
            greedy: true,
            sub: Box::new(Hir::literal("x".as_bytes())),
        });

        let result = compiler.c_repeat_range_min_or_more(&expr, true, 2);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some());
        assert_eq!(patch.unwrap().entry, 0);
    }

    #[test]
    fn test_c_repeat_range_min_or_more_not_greedy() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 2,
            max: None,
            greedy: false,
            sub: Box::new(Hir::literal("y".as_bytes())),
        });

        let result = compiler.c_repeat_range_min_or_more(&expr, false, 2);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some());
        assert_eq!(patch.unwrap().entry, 0);
    }

    #[test]
    fn test_c_repeat_range_min_or_more_empty() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 0,
            max: None,
            greedy: true,
            sub: Box::new(Hir::literal("".as_bytes())),
        });

        let result = compiler.c_repeat_range_min_or_more(&expr, true, 0);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_none());
    }

    #[test]
    fn test_c_repeat_range_min_or_more_invalid() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 1,
            max: None,
            greedy: true,
            sub: Box::new(Hir::literal("z".as_bytes())),
        });

        let result = compiler.c_repeat_range_min_or_more(&expr, true, 2);
        assert!(result.is_ok());
        let patch = result.unwrap();
        assert!(patch.is_some());
    }
}

#[cfg(test)]
mod tests_llm_16_245 {
    use super::*; // Assuming `c_repeat_zero_or_one` is part of the parent module

use crate::*;
    use regex_syntax::hir::{Hir, HirKind, Repetition};

    #[test]
    fn test_c_repeat_zero_or_one_greedy() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 0,
            max: Some(1),
            greedy: true,
            sub: Box::new(Hir::literal("abc".as_bytes())),
        });

        let result = compiler.c_repeat_zero_or_one(&expr, true).unwrap();
        assert!(result.is_some());
        // Add more assertions based on the expected structure of the Patch
    }

    #[test]
    fn test_c_repeat_zero_or_one_lazy() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 0,
            max: Some(1),
            greedy: false,
            sub: Box::new(Hir::literal("abc".as_bytes())),
        });

        let result = compiler.c_repeat_zero_or_one(&expr, false).unwrap();
        assert!(result.is_some());
        // Add more assertions based on the expected structure of the Patch
    }

    #[test]
    fn test_c_repeat_zero_or_one_no_expr() {
        let mut compiler = Compiler::new();
        let expr = Hir::repetition(Repetition {
            min: 0,
            max: Some(1),
            greedy: true,
            sub: Box::new(Hir::empty()),
        });

        let result = compiler.c_repeat_zero_or_one(&expr, true).unwrap();
        assert!(result.is_none()); // Expecting None since subexpression is empty
    }
}

#[cfg(test)]
mod tests_llm_16_246 {
    use super::*;

use crate::*;
    use crate::compile::{Compiler, Error};

    #[test]
    fn test_check_size_success() {
        let compiler = Compiler::new()
            .size_limit(100) // Set a size limit for the compiler
            .size_limit(10 * (1 << 20)); // Set a limit greater than the minimum
        let result = compiler.check_size();
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_size_failure() {
        let mut compiler = Compiler::new()
            .size_limit(100) // Set a size limit for the compiler
            .size_limit(10 * (1 << 20)); // Set a limit greater than the minimum
        compiler.extra_inst_bytes = 200; // Set the size to exceed limit 
        let result = compiler.check_size();
        assert!(result.is_err());
        if let Err(Error::CompiledTooBig(limit)) = result {
            assert_eq!(limit, 100);
        } else {
            panic!("Expected a CompiledTooBig error");
        }
    }
}

#[cfg(test)]
mod tests_llm_16_250 {
    use super::*;

use crate::*;
    use crate::compile::Compiler;
    use regex_syntax::hir::{self, Hir, HirKind};

    #[test]
    fn test_compile_one_valid() {
        let comp = Compiler::new();
        let expr = Hir::literal("abc".as_bytes());
        let result = comp.compile_one(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_one_invalid_utf8() {
        let comp = Compiler::new().only_utf8(true);
        let expr = Hir::literal(vec![0xFF].as_slice()); // Invalid UTF-8
        let result = comp.compile_one(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Syntax(_)));
    }

    #[test]
    fn test_compile_one_word_boundary() {
        let comp = Compiler::new().only_utf8(true);
        let expr = Hir::look(hir::Look::WordAsciiNegate);
        let result = comp.compile_one(&expr);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Syntax(_)));
    }

    #[test]
    fn test_compile_one_word_boundary_unicode_enabled() {
        let comp = Compiler::new().only_utf8(true).bytes(false);
        let expr = Hir::look(hir::Look::WordUnicode);
        let result = comp.compile_one(&expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_one_empty() {
        let comp = Compiler::new();
        let expr = Hir::empty();
        let result = comp.compile_one(&expr);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_251 {
    use super::*;

use crate::*;
    use crate::compile::Compiler;

    #[test]
    fn test_dfa_enable() {
        let compiler = Compiler::new();
        let modified_compiler = compiler.dfa(true);
        assert!(modified_compiler.compiled.is_dfa);
    }

    #[test]
    fn test_dfa_disable() {
        let compiler = Compiler::new();
        let modified_compiler = compiler.dfa(false);
        assert!(!modified_compiler.compiled.is_dfa);
    }

    #[test]
    fn test_dfa_chain() {
        let compiler = Compiler::new();
        let modified_compiler = compiler.dfa(true).dfa(false);
        assert!(!modified_compiler.compiled.is_dfa);
    }
}

#[cfg(test)]
mod tests_llm_16_255 {
    use super::*;

use crate::*;
    use crate::compile::Compiler;

    #[test]
    fn test_compiler_new() {
        let compiler = Compiler::new();

        // Check that the `insts` vector is empty.
        assert!(compiler.insts.is_empty());

        // Check that the `compiled` field is a new `Program`.
        assert_eq!(compiler.compiled.insts.len(), 0);
        
        // Check that the `capture_name_idx` is empty.
        assert!(compiler.capture_name_idx.is_empty());

        // Check that `num_exprs` is initialized to 0.
        assert_eq!(compiler.num_exprs, 0);

        // Check that `size_limit` is set to 10 MB.
        assert_eq!(compiler.size_limit, 10 * (1 << 20));

        // Check `suffix_cache` is initialized properly.
        assert_eq!(compiler.suffix_cache.sparse.len(), 1000);
        assert!(compiler.suffix_cache.dense.is_empty());

        // Check that `utf8_seqs` is initialized properly.
        assert!(compiler.utf8_seqs.is_some());

        // Check that `byte_classes` is a new `ByteClassSet`.
        assert_eq!(compiler.byte_classes.0, [false; 256]);

        // Check that `extra_inst_bytes` is initialized to 0.
        assert_eq!(compiler.extra_inst_bytes, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_263 {
    use super::*;

use crate::*;
    use crate::compile::Compiler;

    #[test]
    fn test_size_limit_set() {
        let compiler = Compiler::new();
        let new_size_limit = 1024;

        let updated_compiler = compiler.size_limit(new_size_limit);
        assert_eq!(updated_compiler.size_limit, new_size_limit);
    }

    #[test]
    fn test_size_limit_initial_value() {
        let compiler = Compiler::new();
        assert_eq!(compiler.size_limit, 10 * (1 << 20)); // default value
    }

    #[test]
    fn test_size_limit_zero() {
        let compiler = Compiler::new();
        let updated_compiler = compiler.size_limit(0);
        assert_eq!(updated_compiler.size_limit, 0);
    }

    #[test]
    fn test_size_limit_large_value() {
        let compiler = Compiler::new();
        let new_size_limit = 1 << 30;

        let updated_compiler = compiler.size_limit(new_size_limit);
        assert_eq!(updated_compiler.size_limit, new_size_limit);
    }
}

#[cfg(test)]
mod tests_llm_16_265 {
    use super::*;

use crate::*;
    use crate::compile::{InstHole, Inst};

    #[test]
    fn test_fill_save() {
        let hole = InstHole::Save { slot: 1 };
        let goto = InstPtr::default(); // Assuming InstPtr has a default implementation
        let result = hole.fill(goto);
        match result {
            Inst::Save(ref save_inst) => {
                assert_eq!(save_inst.slot, 1);
                assert_eq!(save_inst.goto, goto);
            }
            _ => panic!("Expected Inst::Save variant"),
        }
    }

    #[test]
    fn test_fill_empty_look() {
        let hole = InstHole::EmptyLook { look: EmptyLook::StartLine };
        let goto = InstPtr::default();
        let result = hole.fill(goto);
        match result {
            Inst::EmptyLook(ref empty_look_inst) => {
                assert_eq!(empty_look_inst.look, EmptyLook::StartLine);
                assert_eq!(empty_look_inst.goto, goto);
            }
            _ => panic!("Expected Inst::EmptyLook variant"),
        }
    }

    #[test]
    fn test_fill_char() {
        let hole = InstHole::Char { c: 'a' };
        let goto = InstPtr::default();
        let result = hole.fill(goto);
        match result {
            Inst::Char(ref char_inst) => {
                assert_eq!(char_inst.c, 'a');
                assert_eq!(char_inst.goto, goto);
            }
            _ => panic!("Expected Inst::Char variant"),
        }
    }

    #[test]
    fn test_fill_ranges() {
        let hole = InstHole::Ranges { ranges: vec![('a', 'z')] };
        let goto = InstPtr::default();
        let result = hole.fill(goto);
        match result {
            Inst::Ranges(ref ranges_inst) => {
                assert_eq!(ranges_inst.goto, goto);
                assert_eq!(ranges_inst.ranges[0], ('a', 'z'));
            }
            _ => panic!("Expected Inst::Ranges variant"),
        }
    }

    #[test]
    fn test_fill_bytes() {
        let hole = InstHole::Bytes { start: 0x41, end: 0x5A }; // A-Z in ASCII
        let goto = InstPtr::default();
        let result = hole.fill(goto);
        match result {
            Inst::Bytes(ref bytes_inst) => {
                assert_eq!(bytes_inst.start, 0x41);
                assert_eq!(bytes_inst.end, 0x5A);
                assert_eq!(bytes_inst.goto, goto);
            }
            _ => panic!("Expected Inst::Bytes variant"),
        }
    }
}

#[cfg(test)]
mod tests_llm_16_269 {
    use super::*;

use crate::*;
    use crate::compile::{MaybeInst, InstPtr};

    #[test]
    fn test_half_fill_split_goto2() {
        let mut maybe_inst = MaybeInst::Split;
        let goto2: InstPtr = 2; // Example InstPtr value

        maybe_inst.half_fill_split_goto2(goto2);

        if let MaybeInst::Split2(filled) = maybe_inst {
            assert_eq!(filled, goto2);
        } else {
            panic!("Expected MaybeInst::Split2, but got: {:?}", maybe_inst);
        }
    }

    #[test]
    #[should_panic(expected = "must be called on Split instruction")]
    fn test_half_fill_split_goto2_not_split() {
        let mut maybe_inst = MaybeInst::Uncompiled(InstHole::Save { slot: 0 });
        let goto2: InstPtr = 2; // Example InstPtr value

        maybe_inst.half_fill_split_goto2(goto2);
    }
}

#[cfg(test)]
mod tests_llm_16_273 {
    use super::*;

use crate::*;
    use std::vec::Vec;

    #[test]
    fn test_hash() {
        let suffix_cache = SuffixCache::new(10);
        let suffix_key = SuffixCacheKey {
            from_inst: 1,
            start: 10,
            end: 20,
        };
        let hash_value = suffix_cache.hash(&suffix_key);
        assert!(hash_value < 10); // since size of sparse array is 10
    }

    #[test]
    fn test_hash_with_different_keys() {
        let suffix_cache = SuffixCache::new(10);
        
        let suffix_key1 = SuffixCacheKey {
            from_inst: 1,
            start: 10,
            end: 20,
        };
        
        let suffix_key2 = SuffixCacheKey {
            from_inst: 2,
            start: 30,
            end: 40,
        };

        let hash_value1 = suffix_cache.hash(&suffix_key1);
        let hash_value2 = suffix_cache.hash(&suffix_key2);
        
        assert!(hash_value1 < 10);
        assert!(hash_value2 < 10);
        assert!(hash_value1 != hash_value2 || suffix_key1 == suffix_key2); // Ensure they can have same hash
    }
}

#[cfg(test)]
mod tests_llm_16_274 {
    use super::*;

use crate::*;
    
    #[test]
    fn test_suffix_cache_new() {
        // Test initializing SuffixCache with size 0
        let cache_zero = SuffixCache::new(0);
        assert_eq!(cache_zero.sparse.len(), 0);
        assert_eq!(cache_zero.dense.len(), 0);
        
        // Test initializing SuffixCache with a size greater than 0
        let size = 10;
        let cache = SuffixCache::new(size);
        assert_eq!(cache.sparse.len(), size);
        assert_eq!(cache.dense.len(), 0);
    }
}
