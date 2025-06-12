fn duplicate_field(field: &'static str) -> Self {
                Error::custom(format_args!("duplicate field `{}`", field))
            }