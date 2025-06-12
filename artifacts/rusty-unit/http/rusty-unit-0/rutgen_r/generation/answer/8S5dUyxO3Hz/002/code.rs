// Answer 0

#[test]
fn test_path_and_query_none_due_to_scheme_inner_is_none_and_authority_data_not_empty() {
    struct Scheme {
        inner: Option<i32>,
    }

    struct Authority {
        data: String,
    }

    struct PathAndQuery;

    struct Uri {
        scheme: Scheme,
        authority: Authority,
        path_and_query: PathAndQuery,
    }

    impl Uri {
        pub fn path_and_query(&self) -> Option<&PathAndQuery> {
            if !self.scheme.inner.is_none() || self.authority.data.is_empty() {
                Some(&self.path_and_query)
            } else {
                None
            }
        }
    }

    let uri = Uri {
        scheme: Scheme { inner: None },
        authority: Authority { data: String::from("authority_data") },
        path_and_query: PathAndQuery,
    };

    assert_eq!(uri.path_and_query(), None);
}

