type Result<T> = result::Result<T, Error>;
use std::cell::{Cell, RefCell};
use std::result;
use ast::{self, Ast, Span, Visitor};
use hir::{self, Error, ErrorKind, Hir};
use unicode::{self, ClassQuery};
#[derive(Clone, Debug)]
pub struct Translator {
    /// Our call stack, but on the heap.
    stack: RefCell<Vec<HirFrame>>,
    /// The current flag settings.
    flags: Cell<Flags>,
    /// Whether we're allowed to produce HIR that can match arbitrary bytes.
    allow_invalid_utf8: bool,
}
#[derive(Clone, Debug)]
pub struct TranslatorBuilder {
    allow_invalid_utf8: bool,
    flags: Flags,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flags {
    /// The span of this group of flags.
    pub span: Span,
    /// A sequence of flag items. Each item is either a flag or a negation
    /// operator.
    pub items: Vec<FlagsItem>,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}
#[derive(Clone, Debug)]
enum HirFrame {
    /// An arbitrary HIR expression. These get pushed whenever we hit a base
    /// case in the Ast. They get popped after an inductive (i.e., recursive)
    /// step is complete.
    Expr(Hir),
    /// A Unicode character class. This frame is mutated as we descend into
    /// the Ast of a character class (which is itself its own mini recursive
    /// structure).
    ClassUnicode(hir::ClassUnicode),
    /// A byte-oriented character class. This frame is mutated as we descend
    /// into the Ast of a character class (which is itself its own mini
    /// recursive structure).
    ///
    /// Byte character classes are created when Unicode mode (`u`) is disabled.
    /// If `allow_invalid_utf8` is disabled (the default), then a byte
    /// character is only permitted to match ASCII text.
    ClassBytes(hir::ClassBytes),
    /// This is pushed on to the stack upon first seeing any kind of group,
    /// indicated by parentheses (including non-capturing groups). It is popped
    /// upon leaving a group.
    Group {
        /// The old active flags, if any, when this group was opened.
        ///
        /// If this group sets flags, then the new active flags are set to the
        /// result of merging the old flags with the flags introduced by this
        /// group.
        ///
        /// When this group is popped, the active flags should be restored to
        /// the flags set here.
        ///
        /// The "active" flags correspond to whatever flags are set in the
        /// Translator.
        old_flags: Option<Flags>,
    },
    /// This is pushed whenever a concatenation is observed. After visiting
    /// every sub-expression in the concatenation, the translator's stack is
    /// popped until it sees a Concat frame.
    Concat,
    /// This is pushed whenever an alternation is observed. After visiting
    /// every sub-expression in the alternation, the translator's stack is
    /// popped until it sees an Alternation frame.
    Alternation,
}
impl Translator {
    pub fn new() -> Translator {
        TranslatorBuilder::new().build()
    }
    pub fn translate(&mut self, pattern: &str, ast: &Ast) -> Result<Hir> {}
}
impl TranslatorBuilder {
    pub fn new() -> TranslatorBuilder {
        TranslatorBuilder {
            allow_invalid_utf8: false,
            flags: Flags::default(),
        }
    }
    pub fn build(&self) -> Translator {
        Translator {
            stack: RefCell::new(vec![]),
            flags: Cell::new(self.flags),
            allow_invalid_utf8: self.allow_invalid_utf8,
        }
    }
    pub fn allow_invalid_utf8(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn case_insensitive(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn multi_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn swap_greed(&mut self, yes: bool) -> &mut TranslatorBuilder {}
    pub fn unicode(&mut self, yes: bool) -> &mut TranslatorBuilder {}
}
