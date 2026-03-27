use crate::components::position::Position;

pub struct Player {
    pub name : String,
    pub pos : Position
}

impl Player {
    pub fn new(name : &str, x : f32, y : f32) -> Self {
        Self {
            name : name.to_string(),
            pos : Position::new(x, y)
        }
    }
}
