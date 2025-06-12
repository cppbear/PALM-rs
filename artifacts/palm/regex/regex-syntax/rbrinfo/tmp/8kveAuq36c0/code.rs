fn from(err: &'e hir::Error) -> Self {
        Formatter {
            pattern: err.pattern(),
            err: err.kind(),
            span: err.span(),
            aux_span: None,
        }
    }