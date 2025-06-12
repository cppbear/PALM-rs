type Result<T> = result::Result<T, Error>;
use std::cell::{Cell, RefCell};
use std::result;
use ast::{self, Ast, Span, Visitor};
use hir::{self, Error, ErrorKind, Hir};
use unicode::{self, ClassQuery};
#[derive(Clone, Debug)]
struct TranslatorI<'t, 'p> {
    trans: &'t Translator,
    pattern: &'p str,
}
#[derive(Clone, Copy, Debug, Default)]
struct Flags {
    case_insensitive: Option<bool>,
    multi_line: Option<bool>,
    dot_matches_new_line: Option<bool>,
    swap_greed: Option<bool>,
    unicode: Option<bool>,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassBytesRange {
    start: u8,
    end: u8,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassAscii {
    /// The span of this class.
    pub span: Span,
    /// The kind of ASCII class.
    pub kind: ClassAsciiKind,
    /// Whether the class is negated or not. e.g., `[[:alpha:]]` is not negated
    /// but `[[:^alpha:]]` is.
    pub negated: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassSetRange {
    /// The span of this range.
    pub span: Span,
    /// The start of this range.
    pub start: Literal,
    /// The end of this range.
    pub end: Literal,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not.
    ///
    /// Note: be careful when using this attribute. This specifically refers
    /// to whether the class is written as `\p` or `\P`, where the latter
    /// is `negated = true`. However, it also possible to write something like
    /// `\P{scx!=Katakana}` which is actually equivalent to
    /// `\p{scx=Katakana}` and is therefore not actually negated even though
    /// `negated = true` here. To test whether this class is truly negated
    /// or not, use the `is_negated` method.
    pub negated: bool,
    /// The kind of Unicode class.
    pub kind: ClassUnicodeKind,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    /// The span of this literal.
    pub span: Span,
    /// The kind of this literal.
    pub kind: LiteralKind,
    /// The Unicode scalar value corresponding to this literal.
    pub c: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassPerl {
    /// The span of this class.
    pub span: Span,
    /// The kind of Perl class.
    pub kind: ClassPerlKind,
    /// Whether the class is negated or not. e.g., `\d` is not negated but
    /// `\D` is.
    pub negated: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Debug)]
pub struct Translator {
    /// Our call stack, but on the heap.
    stack: RefCell<Vec<HirFrame>>,
    /// The current flag settings.
    flags: Cell<Flags>,
    /// Whether we're allowed to produce HIR that can match arbitrary bytes.
    allow_invalid_utf8: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBracketed {
    /// The span of this class.
    pub span: Span,
    /// Whether this class is negated or not. e.g., `[a]` is not negated but
    /// `[^a]` is.
    pub negated: bool,
    /// The type of this set. A set is either a normal union of things, e.g.,
    /// `[abc]` or a result of applying set operations, e.g., `[\pL--c]`.
    pub kind: ClassSet,
}
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Span {
    /// The start byte offset.
    pub start: Position,
    /// The end byte offset.
    pub end: Position,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the translator's Ast was parsed from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error, derived from the Ast given to the translator.
    span: Span,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error that occurred while translating concrete syntax into abstract
    /// syntax (AST).
    Parse(ast::Error),
    /// An error that occurred while translating abstract syntax into a high
    /// level intermediate representation (HIR).
    Translate(hir::Error),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassAsciiKind {
    /// `[0-9A-Za-z]`
    Alnum,
    /// `[A-Za-z]`
    Alpha,
    /// `[\x00-\x7F]`
    Ascii,
    /// `[ \t]`
    Blank,
    /// `[\x00-\x1F\x7F]`
    Cntrl,
    /// `[0-9]`
    Digit,
    /// `[!-~]`
    Graph,
    /// `[a-z]`
    Lower,
    /// `[ -~]`
    Print,
    /// `[!-/:-@\[-`{-~]`
    Punct,
    /// `[\t\n\v\f\r ]`
    Space,
    /// `[A-Z]`
    Upper,
    /// `[0-9A-Za-z_]`
    Word,
    /// `[0-9A-Fa-f]`
    Xdigit,
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClassSetItem {
    /// An empty item.
    ///
    /// Note that a bracketed character class cannot contain a single empty
    /// item. Empty items can appear when using one of the binary operators.
    /// For example, `[&&]` is the intersection of two empty classes.
    Empty(Span),
    /// A single literal.
    Literal(Literal),
    /// A range between two literals.
    Range(ClassSetRange),
    /// An ASCII character class, e.g., `[:alnum:]` or `[:punct:]`.
    Ascii(ClassAscii),
    /// A Unicode character class, e.g., `\pL` or `\p{Greek}`.
    Unicode(ClassUnicode),
    /// A perl character class, e.g., `\d` or `\W`.
    Perl(ClassPerl),
    /// A bracketed character class set, which may contain zero or more
    /// character ranges and/or zero or more nested classes. e.g.,
    /// `[a-zA-Z\pL]`.
    Bracketed(Box<ClassBracketed>),
    /// A union of items.
    Union(ClassSetUnion),
}
impl<'t, 'p> Visitor for TranslatorI<'t, 'p> {
    type Output = Hir;
    type Err = Error;
    fn finish(self) -> Result<Hir> {}
    fn visit_pre(&mut self, ast: &Ast) -> Result<()> {}
    fn visit_post(&mut self, ast: &Ast) -> Result<()> {}
    fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> Result<()> {}
    fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> Result<()> {
        match *ast {
            ast::ClassSetItem::Empty(_) => {}
            ast::ClassSetItem::Literal(ref x) => {
                if self.flags().unicode() {
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    cls.push(hir::ClassUnicodeRange::new(x.c, x.c));
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    let byte = self.class_literal_byte(x)?;
                    cls.push(hir::ClassBytesRange::new(byte, byte));
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            ast::ClassSetItem::Range(ref x) => {
                if self.flags().unicode() {
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    cls.push(hir::ClassUnicodeRange::new(x.start.c, x.end.c));
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    let start = self.class_literal_byte(&x.start)?;
                    let end = self.class_literal_byte(&x.end)?;
                    cls.push(hir::ClassBytesRange::new(start, end));
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            ast::ClassSetItem::Ascii(ref x) => {
                if self.flags().unicode() {
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    for &(s, e) in ascii_class(&x.kind) {
                        cls.push(hir::ClassUnicodeRange::new(s, e));
                    }
                    self.unicode_fold_and_negate(x.negated, &mut cls);
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    for &(s, e) in ascii_class(&x.kind) {
                        cls.push(hir::ClassBytesRange::new(s as u8, e as u8));
                    }
                    self.bytes_fold_and_negate(&x.span, x.negated, &mut cls)?;
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            ast::ClassSetItem::Unicode(ref x) => {
                let xcls = self.hir_unicode_class(x)?;
                let mut cls = self.pop().unwrap().unwrap_class_unicode();
                cls.union(&xcls);
                self.push(HirFrame::ClassUnicode(cls));
            }
            ast::ClassSetItem::Perl(ref x) => {
                if self.flags().unicode() {
                    let xcls = self.hir_perl_unicode_class(x);
                    let mut cls = self.pop().unwrap().unwrap_class_unicode();
                    cls.union(&xcls);
                    self.push(HirFrame::ClassUnicode(cls));
                } else {
                    let xcls = self.hir_perl_byte_class(x);
                    let mut cls = self.pop().unwrap().unwrap_class_bytes();
                    cls.union(&xcls);
                    self.push(HirFrame::ClassBytes(cls));
                }
            }
            ast::ClassSetItem::Bracketed(ref ast) => {
                if self.flags().unicode() {
                    let mut cls1 = self.pop().unwrap().unwrap_class_unicode();
                    self.unicode_fold_and_negate(ast.negated, &mut cls1);
                    let mut cls2 = self.pop().unwrap().unwrap_class_unicode();
                    cls2.union(&cls1);
                    self.push(HirFrame::ClassUnicode(cls2));
                } else {
                    let mut cls1 = self.pop().unwrap().unwrap_class_bytes();
                    self.bytes_fold_and_negate(&ast.span, ast.negated, &mut cls1)?;
                    let mut cls2 = self.pop().unwrap().unwrap_class_bytes();
                    cls2.union(&cls1);
                    self.push(HirFrame::ClassBytes(cls2));
                }
            }
            ast::ClassSetItem::Union(_) => {}
        }
        Ok(())
    }
    fn visit_class_set_binary_op_pre(
        &mut self,
        _op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
    fn visit_class_set_binary_op_in(
        &mut self,
        _op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
    fn visit_class_set_binary_op_post(
        &mut self,
        op: &ast::ClassSetBinaryOp,
    ) -> Result<()> {}
}
impl<'t, 'p> TranslatorI<'t, 'p> {
    fn new(trans: &'t Translator, pattern: &'p str) -> TranslatorI<'t, 'p> {}
    fn trans(&self) -> &Translator {}
    fn push(&self, frame: HirFrame) {
        self.trans().stack.borrow_mut().push(frame);
    }
    fn pop(&self) -> Option<HirFrame> {
        self.trans().stack.borrow_mut().pop()
    }
    fn error(&self, span: Span, kind: ErrorKind) -> Error {}
    fn flags(&self) -> Flags {
        self.trans().flags.get()
    }
    fn set_flags(&self, ast_flags: &ast::Flags) -> Flags {}
    fn hir_literal(&self, lit: &ast::Literal) -> Result<Hir> {}
    fn literal_to_char(&self, lit: &ast::Literal) -> Result<hir::Literal> {}
    fn hir_from_char(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_from_char_case_insensitive(&self, span: Span, c: char) -> Result<Hir> {}
    fn hir_dot(&self, span: Span) -> Result<Hir> {}
    fn hir_assertion(&self, asst: &ast::Assertion) -> Result<Hir> {}
    fn hir_group(&self, group: &ast::Group, expr: Hir) -> Hir {}
    fn hir_repetition(&self, rep: &ast::Repetition, expr: Hir) -> Hir {}
    fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {
        use ast::ClassUnicodeKind::*;
        if !self.flags().unicode() {
            return Err(self.error(ast_class.span, ErrorKind::UnicodeNotAllowed));
        }
        let query = match ast_class.kind {
            OneLetter(name) => ClassQuery::OneLetter(name),
            Named(ref name) => ClassQuery::Binary(name),
            NamedValue { ref name, ref value, .. } => {
                ClassQuery::ByValue {
                    property_name: name,
                    property_value: value,
                }
            }
        };
        match unicode::class(query) {
            Ok(mut class) => {
                self.unicode_fold_and_negate(ast_class.negated, &mut class);
                Ok(class)
            }
            Err(unicode::Error::PropertyNotFound) => {
                Err(self.error(ast_class.span, ErrorKind::UnicodePropertyNotFound))
            }
            Err(unicode::Error::PropertyValueNotFound) => {
                Err(self.error(ast_class.span, ErrorKind::UnicodePropertyValueNotFound))
            }
        }
    }
    fn hir_perl_unicode_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassUnicode {
        use ast::ClassPerlKind::*;
        use unicode_tables::perl_word::PERL_WORD;
        assert!(self.flags().unicode());
        let mut class = match ast_class.kind {
            Digit => {
                let query = ClassQuery::Binary("Decimal_Number");
                unicode::class(query).unwrap()
            }
            Space => {
                let query = ClassQuery::Binary("Whitespace");
                unicode::class(query).unwrap()
            }
            Word => unicode::hir_class(PERL_WORD),
        };
        if ast_class.negated {
            class.negate();
        }
        class
    }
    fn hir_perl_byte_class(&self, ast_class: &ast::ClassPerl) -> hir::ClassBytes {
        use ast::ClassPerlKind::*;
        assert!(! self.flags().unicode());
        let mut class = match ast_class.kind {
            Digit => hir_ascii_class_bytes(&ast::ClassAsciiKind::Digit),
            Space => hir_ascii_class_bytes(&ast::ClassAsciiKind::Space),
            Word => hir_ascii_class_bytes(&ast::ClassAsciiKind::Word),
        };
        if ast_class.negated {
            class.negate();
        }
        class
    }
    fn unicode_fold_and_negate(&self, negated: bool, class: &mut hir::ClassUnicode) {
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
    }
    fn bytes_fold_and_negate(
        &self,
        span: &Span,
        negated: bool,
        class: &mut hir::ClassBytes,
    ) -> Result<()> {
        if self.flags().case_insensitive() {
            class.case_fold_simple();
        }
        if negated {
            class.negate();
        }
        if !self.trans().allow_invalid_utf8 && !class.is_all_ascii() {
            return Err(self.error(span.clone(), ErrorKind::InvalidUtf8));
        }
        Ok(())
    }
    fn class_literal_byte(&self, ast: &ast::Literal) -> Result<u8> {
        match self.literal_to_char(ast)? {
            hir::Literal::Byte(byte) => Ok(byte),
            hir::Literal::Unicode(ch) => {
                if ch <= 0x7F as char {
                    Ok(ch as u8)
                } else {
                    Err(self.error(ast.span, ErrorKind::UnicodeNotAllowed))
                }
            }
        }
    }
}
impl Flags {
    fn from_ast(ast: &ast::Flags) -> Flags {}
    fn merge(&mut self, previous: &Flags) {}
    fn case_insensitive(&self) -> bool {}
    fn multi_line(&self) -> bool {}
    fn dot_matches_new_line(&self) -> bool {}
    fn swap_greed(&self) -> bool {}
    fn unicode(&self) -> bool {
        self.unicode.unwrap_or(true)
    }
}
impl ClassUnicodeRange {
    pub fn new(start: char, end: char) -> ClassUnicodeRange {
        ClassUnicodeRange::create(start, end)
    }
    pub fn start(&self) -> char {}
    pub fn end(&self) -> char {}
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {
        self.set.push(range);
    }
    pub fn iter(&self) -> ClassUnicodeIter {}
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassUnicode) {
        self.set.union(&other.set);
    }
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {}
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {
        self.set.push(range);
    }
    pub fn iter(&self) -> ClassBytesIter {}
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {
        self.set.union(&other.set);
    }
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_all_ascii(&self) -> bool {}
}
impl HirFrame {
    fn unwrap_expr(self) -> Hir {}
    fn unwrap_class_unicode(self) -> hir::ClassUnicode {
        match self {
            HirFrame::ClassUnicode(cls) => cls,
            _ => {
                panic!(
                    "tried to unwrap Unicode class \
                         from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }
    fn unwrap_class_bytes(self) -> hir::ClassBytes {
        match self {
            HirFrame::ClassBytes(cls) => cls,
            _ => {
                panic!(
                    "tried to unwrap byte class \
                         from HirFrame, got: {:?}",
                    self
                )
            }
        }
    }
    fn unwrap_group(self) -> Option<Flags> {}
}
impl ClassBytesRange {
    pub fn new(start: u8, end: u8) -> ClassBytesRange {
        ClassBytesRange::create(start, end)
    }
    pub fn start(&self) -> u8 {}
    pub fn end(&self) -> u8 {}
}
fn ascii_class(kind: &ast::ClassAsciiKind) -> &'static [(char, char)] {
    use ast::ClassAsciiKind::*;
    type T = &'static [(char, char)];
    match *kind {
        Alnum => {
            const X: T = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
            X
        }
        Alpha => {
            const X: T = &[('A', 'Z'), ('a', 'z')];
            X
        }
        Ascii => {
            const X: T = &[('\x00', '\x7F')];
            X
        }
        Blank => {
            const X: T = &[(' ', '\t')];
            X
        }
        Cntrl => {
            const X: T = &[('\x00', '\x1F'), ('\x7F', '\x7F')];
            X
        }
        Digit => {
            const X: T = &[('0', '9')];
            X
        }
        Graph => {
            const X: T = &[('!', '~')];
            X
        }
        Lower => {
            const X: T = &[('a', 'z')];
            X
        }
        Print => {
            const X: T = &[(' ', '~')];
            X
        }
        Punct => {
            const X: T = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];
            X
        }
        Space => {
            const X: T = &[
                ('\t', '\t'),
                ('\n', '\n'),
                ('\x0B', '\x0B'),
                ('\x0C', '\x0C'),
                ('\r', '\r'),
                (' ', ' '),
            ];
            X
        }
        Upper => {
            const X: T = &[('A', 'Z')];
            X
        }
        Word => {
            const X: T = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];
            X
        }
        Xdigit => {
            const X: T = &[('0', '9'), ('A', 'F'), ('a', 'f')];
            X
        }
    }
}
