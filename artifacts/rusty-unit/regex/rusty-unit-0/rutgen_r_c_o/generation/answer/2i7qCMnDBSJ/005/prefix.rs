// Answer 0

#[test]
fn test_has_subexprs_unicode_class() {
    let unicode_class = Ast::Class(Class::Unicode(ClassUnicode {}));
    unicode_class.has_subexprs();
}

#[test]
fn test_has_subexprs_perl_class() {
    let perl_class = Ast::Class(Class::Perl(ClassPerl {}));
    perl_class.has_subexprs();
}

#[test]
fn test_has_subexprs_bracketed_class() {
    let bracketed_class = Ast::Class(Class::Bracketed(ClassBracketed {}));
    bracketed_class.has_subexprs();
}

