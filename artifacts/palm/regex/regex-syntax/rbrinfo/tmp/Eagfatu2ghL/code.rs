fn hir_group(&self, group: &ast::Group, expr: Hir) -> Hir {
        let kind = match group.kind {
            ast::GroupKind::CaptureIndex(idx) => {
                hir::GroupKind::CaptureIndex(idx)
            }
            ast::GroupKind::CaptureName(ref capname) => {
                hir::GroupKind::CaptureName {
                    name: capname.name.clone(),
                    index: capname.index,
                }
            }
            ast::GroupKind::NonCapturing(_) => hir::GroupKind::NonCapturing,
        };
        Hir::group(hir::Group {
            kind: kind,
            hir: Box::new(expr),
        })
    }