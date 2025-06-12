fn into_class_set_item<P: Borrow<Parser>>(
        self,
        p: &ParserI<P>,
    ) -> Result<ast::ClassSetItem> {
        use ast::ClassSetItem;
        use self::Primitive::*;

        match self {
            Literal(lit) => Ok(ClassSetItem::Literal(lit)),
            Perl(cls) => Ok(ClassSetItem::Perl(cls)),
            Unicode(cls) => Ok(ClassSetItem::Unicode(cls)),
            x => Err(p.error(*x.span(), ast::ErrorKind::ClassEscapeInvalid)),
        }
    }