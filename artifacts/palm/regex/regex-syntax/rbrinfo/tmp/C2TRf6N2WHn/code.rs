fn canonical_binary(&self, name: &str) -> Result<CanonicalClassQuery> {
        let norm = normalize(name);

        if let Some(canon) = canonical_prop(&norm) {
            return Ok(CanonicalClassQuery::Binary(canon));
        }
        if let Some(canon) = canonical_gencat(&norm) {
            return Ok(CanonicalClassQuery::GeneralCategory(canon));
        }
        if let Some(canon) = canonical_script(&norm) {
            return Ok(CanonicalClassQuery::Script(canon));
        }
        Err(Error::PropertyNotFound)
    }