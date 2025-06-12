fn from_ast(ast: &ast::Flags) -> Flags {
        let mut flags = Flags::default();
        let mut enable = true;
        for item in &ast.items {
            match item.kind {
                ast::FlagsItemKind::Negation => {
                    enable = false;
                }
                ast::FlagsItemKind::Flag(ast::Flag::CaseInsensitive) => {
                    flags.case_insensitive = Some(enable);
                }
                ast::FlagsItemKind::Flag(ast::Flag::MultiLine) => {
                    flags.multi_line = Some(enable);
                }
                ast::FlagsItemKind::Flag(ast::Flag::DotMatchesNewLine) => {
                    flags.dot_matches_new_line = Some(enable);
                }
                ast::FlagsItemKind::Flag(ast::Flag::SwapGreed) => {
                    flags.swap_greed = Some(enable);
                }
                ast::FlagsItemKind::Flag(ast::Flag::Unicode) => {
                    flags.unicode = Some(enable);
                }
                ast::FlagsItemKind::Flag(ast::Flag::IgnoreWhitespace) => {}
            }
        }
        flags
    }