fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], output: &mut [u8]) -> Result<usize, DecodeError>
        where
            E: Engine + ?Sized,
        {
            engine
                .internal_decode(
                    input_bytes,
                    output,
                    engine.internal_decoded_len_estimate(input_bytes.len()),
                )
                .map(|dm| dm.decoded_len)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        panic!("Output slice is too small")
                    }
                })
        }

        inner(self, input.as_ref(), output)
    }