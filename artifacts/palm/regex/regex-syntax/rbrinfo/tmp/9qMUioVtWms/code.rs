fn from(err: &'e ast::Error) -> Self {
        Formatter {
            pattern: err.pattern(),
            err: err.kind(),
            span: err.span(),
            aux_span: err.auxiliary_span(),
        }
    }