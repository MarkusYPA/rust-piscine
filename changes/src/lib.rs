#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: alias.to_owned(),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    // use iter_mut to get mutable reference
    match lights
        .iter_mut()
        .find(|l| l.alias == alias) {
            Some(l) => l.brightness = value,
            None => return,
        }
    // Same could be done with: if let Some(l) = lights.ite...
}
