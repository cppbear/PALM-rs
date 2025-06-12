fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use `u16::fmt` so that it respects any formatting flags that
        // may have been set (like padding, align, etc).
        fmt::Display::fmt(&self.port, f)
    }