fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter
                    .debug_struct(stringify!($name))
                    .field("value", &self.value)
                    .finish()
            }