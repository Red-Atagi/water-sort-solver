#[derive(Clone)]
pub struct Liquid {
    color: String,
    volume: u32,
}

impl Liquid {
    pub fn new(color: String, volume: u32) -> Self {
        Liquid { color, volume }
    }

    pub fn color(&self) -> &String {
        &self.color
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }
}