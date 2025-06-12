use std::fmt;
use ast::{self, Ast};
struct HeapVisitor<'a> {
    /// A stack of `Ast` nodes. This is roughly analogous to the call stack
    /// used in a typical recursive visitor.
    stack: Vec<(&'a Ast, Frame<'a>)>,
    /// Similar to the `Ast` stack above, but is used only for character
    /// classes. In particular, character classes embed their own mini
    /// recursive syntax.
    stack_class: Vec<(ClassInduct<'a>, ClassFrame<'a>)>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ast {
    /// An empty regex that matches everything.
    Empty(Span),
    /// A set of flags, e.g., `(?is)`.
    Flags(SetFlags),
    /// A single character literal, which includes escape sequences.
    Literal(Literal),
    /// The "any character" class.
    Dot(Span),
    /// A single zero-width assertion.
    Assertion(Assertion),
    /// A single character class. This includes all forms of character classes
    /// except for `.`. e.g., `\d`, `\pN`, `[a-z]` and `[[:alpha:]]`.
    Class(Class),
    /// A repetition operator applied to an arbitrary regular expression.
    Repetition(Repetition),
    /// A grouped regular expression.
    Group(Group),
    /// An alternation of regular expressions.
    Alternation(Alternation),
    /// A concatenation of regular expressions.
    Concat(Concat),
}
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {
        HeapVisitor {
            stack: vec![],
            stack_class: vec![],
        }
    }
    fn visit<V: Visitor>(
        &mut self,
        mut ast: &'a Ast,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {}
    fn induct<V: Visitor>(
        &mut self,
        ast: &'a Ast,
        visitor: &mut V,
    ) -> Result<Option<Frame<'a>>, V::Err> {}
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
    fn visit_class<V: Visitor>(
        &mut self,
        ast: &'a ast::ClassBracketed,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn visit_class_pre<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn visit_class_post<V: Visitor>(
        &self,
        ast: &ClassInduct<'a>,
        visitor: &mut V,
    ) -> Result<(), V::Err> {}
    fn induct_class(&self, ast: &ClassInduct<'a>) -> Option<ClassFrame<'a>> {}
    fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>> {}
}
pub fn visit<V: Visitor>(ast: &Ast, visitor: V) -> Result<V::Output, V::Err> {
    HeapVisitor::new().visit(ast, visitor)
}
