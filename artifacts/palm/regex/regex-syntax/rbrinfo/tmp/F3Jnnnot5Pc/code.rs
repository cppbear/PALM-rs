fn fmt_group_pre(&mut self, ast: &ast::Group) -> fmt::Result {
        use ast::GroupKind::*;
        match ast.kind {
            CaptureIndex(_) => self.wtr.write_str("("),
            CaptureName(ref x) => {
                self.wtr.write_str("(?P<")?;
                self.wtr.write_str(&x.name)?;
                self.wtr.write_str(">")?;
                Ok(())
            }
            NonCapturing(ref flags) => {
                self.wtr.write_str("(?")?;
                self.fmt_flags(flags)?;
                self.wtr.write_str(":")?;
                Ok(())
            }
        }
    }