fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use self::Unexpected::*;
        match *self {
            Bool(b) => write!(formatter, "boolean `{}`", b),
            Unsigned(i) => write!(formatter, "integer `{}`", i),
            Signed(i) => write!(formatter, "integer `{}`", i),
            Float(f) => write!(formatter, "floating point `{}`", WithDecimalPoint(f)),
            Char(c) => write!(formatter, "character `{}`", c),
            Str(s) => write!(formatter, "string {:?}", s),
            Bytes(_) => formatter.write_str("byte array"),
            Unit => formatter.write_str("unit value"),
            Option => formatter.write_str("Option value"),
            NewtypeStruct => formatter.write_str("newtype struct"),
            Seq => formatter.write_str("sequence"),
            Map => formatter.write_str("map"),
            Enum => formatter.write_str("enum"),
            UnitVariant => formatter.write_str("unit variant"),
            NewtypeVariant => formatter.write_str("newtype variant"),
            TupleVariant => formatter.write_str("tuple variant"),
            StructVariant => formatter.write_str("struct variant"),
            Other(other) => formatter.write_str(other),
        }
    }