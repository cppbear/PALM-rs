fn add_capture_name(&self, cap: &ast::CaptureName) -> Result<()> {
        let mut names = self.parser().capture_names.borrow_mut();
        match names.binary_search_by_key(
            &cap.name.as_str(),
            |c| c.name.as_str(),
        ) {
            Err(i) => {
                names.insert(i, cap.clone());
                Ok(())
            }
            Ok(i) => {
                Err(self.error(cap.span, ast::ErrorKind::GroupNameDuplicate {
                    original: names[i].span,
                }))
            }
        }
    }