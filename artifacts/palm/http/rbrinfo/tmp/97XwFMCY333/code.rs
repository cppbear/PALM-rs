fn from(src: Uri) -> Self {
        let path_and_query = if src.has_path() {
            Some(src.path_and_query)
        } else {
            None
        };

        let scheme = match src.scheme.inner {
            Scheme2::None => None,
            _ => Some(src.scheme),
        };

        let authority = if src.authority.data.is_empty() {
            None
        } else {
            Some(src.authority)
        };

        Parts {
            scheme,
            authority,
            path_and_query,
            _priv: (),
        }
    }