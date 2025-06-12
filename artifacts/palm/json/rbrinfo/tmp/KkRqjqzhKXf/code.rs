fn next(&mut self) -> Result<Option<u8>> {
        match self.ch.take() {
            Some(ch) => {
                #[cfg(feature = "raw_value")]
                {
                    if let Some(buf) = &mut self.raw_buffer {
                        buf.push(ch);
                    }
                }
                Ok(Some(ch))
            }
            None => match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(ch)) => {
                    #[cfg(feature = "raw_value")]
                    {
                        if let Some(buf) = &mut self.raw_buffer {
                            buf.push(ch);
                        }
                    }
                    Ok(Some(ch))
                }
                None => Ok(None),
            },
        }
    }