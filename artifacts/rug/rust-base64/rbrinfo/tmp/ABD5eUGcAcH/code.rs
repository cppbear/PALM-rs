fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
        fn inner<E>(engine: &E, input_bytes: &[u8], output_buf: &mut String)
        where
            E: Engine + ?Sized,
        {
            let mut sink = chunked_encoder::StringSink::new(output_buf);

            chunked_encoder::ChunkedEncoder::new(engine)
                .encode(input_bytes, &mut sink)
                .expect("Writing to a String shouldn't fail");
        }

        inner(self, input.as_ref(), output_buf);
    }