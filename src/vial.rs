use crate::liquid::Liquid;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vial {
    capacity: u32,
    colors: Vec<String>,
}

impl Vial {
    pub fn new(capacity: u32) -> Self {
        Vial {
            capacity,
            colors: Vec::new(),
        }
    }

    // if liquids exceed capacity, they will be truncated
    pub fn from_liquids(capacity: u32, liquids: Vec<Liquid>) -> Self {
        let mut vial = Vial { capacity, colors: Vec::new() };

        for liquid in liquids {
            let _ = vial.pour_in(liquid);
        }
        vial.colors.truncate(capacity as usize);
        vial
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn space_left(&self) -> u32 {
        self.capacity - self.colors.len() as u32
    }

    pub fn volume(&self) -> u32 {
        self.colors.len() as u32
    }

    pub fn pour_in(&mut self, liquid: Liquid) -> Result<(), String> {
        if self.space_left() < liquid.volume() {
            Err("Not enough space in vial".to_string())
        } else {
            for _ in 0..liquid.volume() {
                self.colors.push(liquid.color().clone());
            }
            Ok(())
        }
    }

    pub fn pour_out(&mut self) -> Result<Liquid, String> {
        if self.volume() == 0 {
            Err("Vial is empty".to_string())
        } else {
            let top_color = self.colors.last().unwrap().clone();
            let count = self.top_color_count();
            for _ in 0..count {
                self.colors.pop();
            }
            Ok(Liquid::new(top_color, count))
        }
    }

    pub fn top_color(&self) -> Option<&String> {
        self.colors.last()
    }

    pub fn top_color_count(&self) -> u32 {
        if let Some(top_color) = self.top_color() {
            self.colors.iter().rev().take_while(|&c| c == top_color).count() as u32
        } else {
            0
        }
    }
}