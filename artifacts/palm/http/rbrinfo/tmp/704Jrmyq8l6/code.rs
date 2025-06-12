fn has_path(&self) -> bool {
        !self.path_and_query.data.is_empty() || !self.scheme.inner.is_none()
    }