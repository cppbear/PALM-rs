fn set_yellow(&mut self) {
        if let Danger::Green = *self {
            *self = Danger::Yellow;
        }
    }