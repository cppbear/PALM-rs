// Answer 0

fn test_visit_pre_empty() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Empty();
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_flags() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Flags();
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_dot() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Dot();
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_literal() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Literal();
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_assertion() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Assertion();
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_unicode_class() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Class(Class::Unicode("unicode".into()));
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

fn test_visit_pre_perl_class() {
    struct Visitor {
        depth: usize,
    }
    
    impl Visitor {
        fn increment_depth(&mut self, _span: &str) -> Result<(), &'static str> {
            self.depth += 1;
            Ok(())
        }
        
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), &'static str> {
            let span = match *ast {
                Ast::Empty(_) | Ast::Flags(_) | Ast::Literal(_) | Ast::Dot(_) |
                Ast::Assertion(_) | Ast::Class(Class::Unicode(_)) | Ast::Class(Class::Perl(_)) => {
                    return Ok(());
                }
                Ast::Class(Class::Bracketed(ref x)) => &x.span,
                Ast::Repetition(ref x) => &x.span,
                Ast::Group(ref x) => &x.span,
                Ast::Alternation(ref x) => &x.span,
                Ast::Concat(ref x) => &x.span,
            };
            self.increment_depth(span)
        }
    }
    
    enum Ast {
        Empty(),
        Flags(),
        Literal(),
        Dot(),
        Assertion(),
        Class(Class),
    }
    
    enum Class {
        Unicode(String),
        Perl(String),
        Bracketed(Bracketed),
    }
    
    struct Bracketed {
        span: String,
    }
    
    let mut visitor = Visitor { depth: 0 };
    let ast = Ast::Class(Class::Perl("perl".into()));
    let result = visitor.visit_pre(&ast);
    assert_eq!(result, Ok(()));
}

