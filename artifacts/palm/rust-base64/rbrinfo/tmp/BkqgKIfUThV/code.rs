fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> String
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(input_bytes.len(), engine.config().encode_padding())
                .expect("integer overflow when calculating buffer size");

            let mut buf = vec![0; encoded_size];

            encode_with_padding(input_bytes, &mut buf[..], engine, encoded_size);

            String::from_utf8(buf).expect("Invalid UTF8")
        }

        inner(self, input.as_ref())
    }