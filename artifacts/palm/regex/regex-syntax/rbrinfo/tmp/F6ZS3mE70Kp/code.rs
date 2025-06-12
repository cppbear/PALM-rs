fn hir_unicode_class(
        &self,
        ast_class: &ast::ClassUnicode,
    ) -> Result<hir::ClassUnicode> {
        use ast::ClassUnicodeKind::*;

        if !self.flags().unicode() {
            return Err(self.error(
                ast_class.span,
                ErrorKind::UnicodeNotAllowed,
            ));
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
                Err(self.error(
                    ast_class.span,
                    ErrorKind::UnicodePropertyNotFound,
                ))
            }
            Err(unicode::Error::PropertyValueNotFound) => {
                Err(self.error(
                    ast_class.span,
                    ErrorKind::UnicodePropertyValueNotFound,
                ))
            }
        }
    }