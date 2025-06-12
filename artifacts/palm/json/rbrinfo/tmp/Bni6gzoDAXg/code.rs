fn from(value: ParserNumber) -> Self {
        let n = match value {
            ParserNumber::F64(f) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::Float(f)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    ryu::Buffer::new().format_finite(f).to_owned()
                }
            }
            ParserNumber::U64(u) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::PosInt(u)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    itoa::Buffer::new().format(u).to_owned()
                }
            }
            ParserNumber::I64(i) => {
                #[cfg(not(feature = "arbitrary_precision"))]
                {
                    N::NegInt(i)
                }
                #[cfg(feature = "arbitrary_precision")]
                {
                    itoa::Buffer::new().format(i).to_owned()
                }
            }
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(s) => s,
        };
        Number { n }
    }