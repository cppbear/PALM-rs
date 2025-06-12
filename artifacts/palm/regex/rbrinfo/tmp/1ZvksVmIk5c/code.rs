fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Captures").field(&CapturesDebug(self)).finish()
    }