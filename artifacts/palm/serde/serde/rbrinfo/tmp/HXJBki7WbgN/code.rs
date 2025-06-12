fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let mut seq = tri!(serializer.serialize_tuple($len));
                    for e in self {
                        tri!(seq.serialize_element(e));
                    }
                    seq.end()
                }