fn decode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeSliceError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            output: &mut [u8],
        ) -> Result<usize, DecodeSliceError>
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
        }

        inner(self, input.as_ref(), output)
    }