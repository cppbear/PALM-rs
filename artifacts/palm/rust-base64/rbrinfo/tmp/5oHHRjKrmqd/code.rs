fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> Result<Vec<u8>, DecodeError>
        where
            E: Engine + ?Sized,
        {
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());
            let mut buffer = vec![0; estimate.decoded_len_estimate()];

            let bytes_written = engine
                .internal_decode(input_bytes, &mut buffer, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;

            buffer.truncate(bytes_written);

            Ok(buffer)
        }

        inner(self, input.as_ref())
    }