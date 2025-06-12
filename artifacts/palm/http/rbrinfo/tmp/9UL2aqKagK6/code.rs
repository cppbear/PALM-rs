pub(crate) fn from_str(bytes: T) -> Result<Self, InvalidUri> {
        bytes
            .as_ref()
            .parse::<u16>()
            .map(|port| Port { port, repr: bytes })
            .map_err(|_| ErrorKind::InvalidPort.into())
    }