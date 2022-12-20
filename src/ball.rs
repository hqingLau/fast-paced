use std::sync::{Mutex, self};

pub enum order {
    LEFT,
    RIGHT,
    DOWN,
    UP
}

// impl Clone for order {
//     fn clone(&self) -> Self {
//         self.clone()
//     }
// }

pub struct Ball {
    pub x: f64,
    pub y: f64,
    pub nextx: f64,
    pub nexty: f64,
    pub radius: f64,
    pub orders: Vec<order>,
    pub lock: sync::Mutex<usize>,
}
impl Ball {
    pub fn default(x: f64, y: f64) -> Self {
        Self { x, y, radius: 100.0,nextx: x,nexty: y, orders: vec![],lock: Mutex::new(0)}
    }

    pub fn movex(&mut self, x: f64)   {
        self.x += x;
    }

    pub fn movey(&mut self, y: f64) {
        self.y += y;
    }
}
