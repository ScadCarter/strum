pub enum Tab {
    Brightness,
    Bluetooth,
    // Device
}

impl Tab {
    pub fn next(&self) -> Self {
        use Tab::*;

        match self {
            Brightness => Bluetooth,
            Bluetooth => Brightness,
        }
    }
}

impl std::fmt::Display for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Tab::Brightness => "Brightness",
            Tab::Bluetooth => "Bluetooth",
        };

        write!(f, "{}", value)
    }
}
