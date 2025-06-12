pub fn port_u16(&self) -> Option<u16> {
        self.port().map(|p| p.as_u16())
    }