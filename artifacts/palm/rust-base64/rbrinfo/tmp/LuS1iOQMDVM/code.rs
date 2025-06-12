fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], buffer: &mut Vec<u8>) -> Result<(), DecodeError>
        where
            E: Engine + ?Sized,
        {
            let starting_output_len = buffer.len();
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());

            let total_len_estimate = estimate
                .decoded_len_estimate()
                .checked_add(starting_output_len)
                .expect("Overflow when calculating output buffer length");

            buffer.resize(total_len_estimate, 0);

            let buffer_slice = &mut buffer.as_mut_slice()[starting_output_len..];

            let bytes_written = engine
                .internal_decode(input_bytes, buffer_slice, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;

            buffer.truncate(starting_output_len + bytes_written);

            Ok(())
        }

        inner(self, input.as_ref(), buffer)
    }