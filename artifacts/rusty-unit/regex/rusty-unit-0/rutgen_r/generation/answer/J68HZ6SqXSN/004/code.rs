// Answer 0

#[test]
fn test_canonicalize_script_property_value_not_found() {
    struct PropertyQuery;

    impl PropertyQuery {
        fn canonical_binary(&self, _name: &str) -> Result<CanonicalClassQuery, Error> {
            Ok(CanonicalClassQuery::Binary(String::from("binary")))
        }

        fn canonicalize(&self) -> Result<CanonicalClassQuery, Error> {
            match self {
                PropertyQuery => {
                    let property_name = String::from("Script");
                    let property_value = String::from("Unknown_Script");
                    let canon_name = match canonical_prop(&property_name) {
                        None => return Err(Error::PropertyNotFound),
                        Some(canon_name) => canon_name,
                    };

                    if canon_name == "Script" {
                        // This should trigger the panic condition as `canonical_script` returns None
                        let canon = canonical_script(&property_value).ok_or(Error::PropertyValueNotFound)?;
                        return Ok(CanonicalClassQuery::Script(canon));
                    }

                    Err(Error::PropertyValueNotFound)
                }
            }
        }
    }

    let query = PropertyQuery;
    let result = query.canonicalize();
    assert_eq!(result, Err(Error::PropertyValueNotFound));
}

