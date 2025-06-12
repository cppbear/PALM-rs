// Answer 0

#[derive(Debug)]
struct Flags {
    case_insensitive: bool,
}

impl Flags {
    fn from_ast(ast_flags: &ast::Flags) -> Self {
        Flags {
            case_insensitive: ast_flags.case_insensitive,
        }
    }

    fn merge(&mut self, other: &Flags) {
        self.case_insensitive |= other.case_insensitive;
    }
}

mod ast {
    #[derive(Debug)]
    pub struct Flags {
        pub case_insensitive: bool,
    }
}

struct Translator {
    flags: Flags,
}

impl Translator {
    fn flags(&self) -> Flags {
        self.flags
    }

    fn trans(&self) -> &Translator {
        self
    }
}

#[test]
fn test_set_flags() {
    let initial_flags = Flags { case_insensitive: false };
    let new_ast_flags = ast::Flags { case_insensitive: true };
    
    let translator = Translator { flags: initial_flags };
    let old_flags = translator.set_flags(&new_ast_flags);
    
    assert_eq!(old_flags.case_insensitive, false);
    assert_eq!(translator.flags.case_insensitive, true);
}

#[test]
fn test_set_flags_multiple() {
    let initial_flags = Flags { case_insensitive: true };
    let new_ast_flags = ast::Flags { case_insensitive: true };
    
    let translator = Translator { flags: initial_flags };
    let old_flags = translator.set_flags(&new_ast_flags);
    
    assert_eq!(old_flags.case_insensitive, true);
    assert_eq!(translator.flags.case_insensitive, true);
}

#[test]
fn test_set_flags_no_change() {
    let initial_flags = Flags { case_insensitive: false };
    let new_ast_flags = ast::Flags { case_insensitive: false };
    
    let translator = Translator { flags: initial_flags };
    let old_flags = translator.set_flags(&new_ast_flags);
    
    assert_eq!(old_flags.case_insensitive, false);
    assert_eq!(translator.flags.case_insensitive, false);
}

