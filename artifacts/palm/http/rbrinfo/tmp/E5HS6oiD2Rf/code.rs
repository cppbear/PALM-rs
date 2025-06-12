pub fn from_parts(src: Parts) -> Result<Uri, InvalidUriParts> {
        if src.scheme.is_some() {
            if src.authority.is_none() {
                return Err(ErrorKind::AuthorityMissing.into());
            }

            if src.path_and_query.is_none() {
                return Err(ErrorKind::PathAndQueryMissing.into());
            }
        } else if src.authority.is_some() && src.path_and_query.is_some() {
            return Err(ErrorKind::SchemeMissing.into());
        }

        let scheme = match src.scheme {
            Some(scheme) => scheme,
            None => Scheme {
                inner: Scheme2::None,
            },
        };

        let authority = match src.authority {
            Some(authority) => authority,
            None => Authority::empty(),
        };

        let path_and_query = match src.path_and_query {
            Some(path_and_query) => path_and_query,
            None => PathAndQuery::empty(),
        };

        Ok(Uri {
            scheme,
            authority,
            path_and_query,
        })
    }