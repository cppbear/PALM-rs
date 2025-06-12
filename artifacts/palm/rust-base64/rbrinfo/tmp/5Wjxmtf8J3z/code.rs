fn decode_to_buf(&mut self, b64_len_to_decode: usize, buf: &mut [u8]) -> io::Result<usize> {
        debug_assert!(self.b64_len >= b64_len_to_decode);
        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
        debug_assert!(!buf.is_empty());

        let b64_to_decode = &self.b64_buffer[self.b64_offset..self.b64_offset + b64_len_to_decode];
        let decode_metadata = self
            .engine
            .internal_decode(
                b64_to_decode,
                buf,
                self.engine.internal_decoded_len_estimate(b64_len_to_decode),
            )
            .map_err(|dse| match dse {
                DecodeSliceError::DecodeError(de) => {
                    match de {
                        DecodeError::InvalidByte(offset, byte) => {
                            match (byte, self.padding_offset) {
                                // if there was padding in a previous block of decoding that happened to
                                // be correct, and we now find more padding that happens to be incorrect,
                                // to be consistent with non-reader decodes, record the error at the first
                                // padding
                                (PAD_BYTE, Some(first_pad_offset)) => {
                                    DecodeError::InvalidByte(first_pad_offset, PAD_BYTE)
                                }
                                _ => {
                                    DecodeError::InvalidByte(self.input_consumed_len + offset, byte)
                                }
                            }
                        }
                        DecodeError::InvalidLength(len) => {
                            DecodeError::InvalidLength(self.input_consumed_len + len)
                        }
                        DecodeError::InvalidLastSymbol(offset, byte) => {
                            DecodeError::InvalidLastSymbol(self.input_consumed_len + offset, byte)
                        }
                        DecodeError::InvalidPadding => DecodeError::InvalidPadding,
                    }
                }
                DecodeSliceError::OutputSliceTooSmall => {
                    unreachable!("buf is sized correctly in calling code")
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if let Some(offset) = self.padding_offset {
            // we've already seen padding
            if decode_metadata.decoded_len > 0 {
                // we read more after already finding padding; report error at first padding byte
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    DecodeError::InvalidByte(offset, PAD_BYTE),
                ));
            }
        }

        self.padding_offset = self.padding_offset.or(decode_metadata
            .padding_offset
            .map(|offset| self.input_consumed_len + offset));
        self.input_consumed_len += b64_len_to_decode;
        self.b64_offset += b64_len_to_decode;
        self.b64_len -= b64_len_to_decode;

        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);

        Ok(decode_metadata.decoded_len)
    }