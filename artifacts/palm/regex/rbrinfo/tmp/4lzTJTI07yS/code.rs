fn c(&mut self, expr: &Hir) -> Result {
        use prog;
        use syntax::hir::HirKind::*;

        self.check_size()?;
        match *expr.kind() {
            Empty => Ok(Patch { hole: Hole::None, entry: self.insts.len() }),
            Literal(hir::Literal::Unicode(c)) => {
                self.c_literal(&[c])
            }
            Literal(hir::Literal::Byte(b)) => {
                assert!(self.compiled.uses_bytes());
                self.c_bytes(&[b])
            }
            Class(hir::Class::Unicode(ref cls)) => {
                self.c_class(cls.ranges())
            }
            Class(hir::Class::Bytes(ref cls)) => {
                if self.compiled.uses_bytes() {
                    self.c_class_bytes(cls.ranges())
                } else {
                    assert!(cls.is_all_ascii());
                    let mut char_ranges = vec![];
                    for r in cls.iter() {
                        let (s, e) = (r.start() as char, r.end() as char);
                        char_ranges.push(hir::ClassUnicodeRange::new(s, e));
                    }
                    self.c_class(&char_ranges)
                }
            }
            Anchor(hir::Anchor::StartLine) if self.compiled.is_reverse => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::EndLine)
            }
            Anchor(hir::Anchor::StartLine) => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::StartLine)
            }
            Anchor(hir::Anchor::EndLine) if self.compiled.is_reverse => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::StartLine)
            }
            Anchor(hir::Anchor::EndLine) => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::EndLine)
            }
            Anchor(hir::Anchor::StartText) if self.compiled.is_reverse => {
                self.c_empty_look(prog::EmptyLook::EndText)
            }
            Anchor(hir::Anchor::StartText) => {
                self.c_empty_look(prog::EmptyLook::StartText)
            }
            Anchor(hir::Anchor::EndText) if self.compiled.is_reverse => {
                self.c_empty_look(prog::EmptyLook::StartText)
            }
            Anchor(hir::Anchor::EndText) => {
                self.c_empty_look(prog::EmptyLook::EndText)
            }
            WordBoundary(hir::WordBoundary::Unicode) => {
                self.compiled.has_unicode_word_boundary = true;
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::WordBoundary)
            }
            WordBoundary(hir::WordBoundary::UnicodeNegate) => {
                self.compiled.has_unicode_word_boundary = true;
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::NotWordBoundary)
            }
            WordBoundary(hir::WordBoundary::Ascii) => {
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::WordBoundaryAscii)
            }
            WordBoundary(hir::WordBoundary::AsciiNegate) => {
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::NotWordBoundaryAscii)
            }
            Group(ref g) => {
                match g.kind {
                    hir::GroupKind::NonCapturing => self.c(&g.hir),
                    hir::GroupKind::CaptureIndex(index) => {
                        if index as usize >= self.compiled.captures.len() {
                            self.compiled.captures.push(None);
                        }
                        self.c_capture(2 * index as usize, &g.hir)
                    }
                    hir::GroupKind::CaptureName { index, ref name } => {
                        if index as usize >= self.compiled.captures.len() {
                            let n = name.to_string();
                            self.compiled.captures.push(Some(n.clone()));
                            self.capture_name_idx.insert(n, index as usize);
                        }
                        self.c_capture(2 * index as usize, &g.hir)
                    }
                }
            }
            Concat(ref es) => {
                if self.compiled.is_reverse {
                    self.c_concat(es.iter().rev())
                } else {
                    self.c_concat(es)
                }
            }
            Alternation(ref es) => self.c_alternate(&**es),
            Repetition(ref rep) => self.c_repeat(rep),
        }
    }