fn into_class_literal<P: Borrow<Parser>>(
        self,
        p: &ParserI<P>,
    ) -> Result<ast::Literal> {
        use self::Primitive::*;

        match self {
            Literal(lit) => Ok(lit),
            x => Err(p.error(*x.span(), ast::ErrorKind::ClassRangeLiteral)),
        }
    }