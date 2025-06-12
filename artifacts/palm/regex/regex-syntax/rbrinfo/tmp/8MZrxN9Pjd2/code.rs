fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
        match *hir.kind() {
            HirKind::Empty
            | HirKind::Repetition(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {}
            HirKind::Literal(hir::Literal::Unicode(c)) => {
                self.write_literal_char(c)?;
            }
            HirKind::Literal(hir::Literal::Byte(b)) => {
                self.write_literal_byte(b)?;
            }
            HirKind::Class(hir::Class::Unicode(ref cls)) => {
                self.wtr.write_str("[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_char(range.start())?;
                    } else {
                        self.write_literal_char(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_char(range.end())?;
                    }
                }
                self.wtr.write_str("]")?;
            }
            HirKind::Class(hir::Class::Bytes(ref cls)) => {
                self.wtr.write_str("(?-u:[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_class_byte(range.start())?;
                    } else {
                        self.write_literal_class_byte(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_class_byte(range.end())?;
                    }
                }
                self.wtr.write_str("])")?;
            }
            HirKind::Anchor(hir::Anchor::StartLine) => {
                self.wtr.write_str("(?m:^)")?;
            }
            HirKind::Anchor(hir::Anchor::EndLine) => {
                self.wtr.write_str("(?m:$)")?;
            }
            HirKind::Anchor(hir::Anchor::StartText) => {
                self.wtr.write_str(r"\A")?;
            }
            HirKind::Anchor(hir::Anchor::EndText) => {
                self.wtr.write_str(r"\z")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::Unicode) => {
                self.wtr.write_str(r"\b")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate) => {
                self.wtr.write_str(r"\B")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::Ascii) => {
                self.wtr.write_str(r"(?-u:\b)")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::AsciiNegate) => {
                self.wtr.write_str(r"(?-u:\B)")?;
            }
            HirKind::Group(ref x) => {
                match x.kind {
                    hir::GroupKind::CaptureIndex(_) => {
                        self.wtr.write_str("(")?;
                    }
                    hir::GroupKind::CaptureName { ref name, .. } => {
                        write!(self.wtr, "(?P<{}>", name)?;
                    }
                    hir::GroupKind::NonCapturing => {
                        self.wtr.write_str("(?:")?;
                    }
                }
            }
        }
        Ok(())
    }