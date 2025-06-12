fn description(&self) -> &str {
        use self::ErrorKind::*;
        match *self {
            UnicodeNotAllowed => "Unicode not allowed here",
            InvalidUtf8 => "pattern can match invalid UTF-8",
            UnicodePropertyNotFound => "Unicode property not found",
            UnicodePropertyValueNotFound => "Unicode property value not found",
            EmptyClassNotAllowed => "empty character classes are not allowed",
            _ => unreachable!(),
        }
    }