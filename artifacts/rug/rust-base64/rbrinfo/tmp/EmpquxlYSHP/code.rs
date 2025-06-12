fn encode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output_buf: &mut [u8],
    ) -> Result<usize, EncodeSliceError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError>
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(input_bytes.len(), engine.config().encode_padding())
                .expect("usize overflow when calculating buffer size");

            if output_buf.len() < encoded_size {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }

            let b64_output = &mut output_buf[0..encoded_size];

            encode_with_padding(input_bytes, b64_output, engine, encoded_size);

            Ok(encoded_size)
        }

        inner(self, input.as_ref(), output_buf)
    }