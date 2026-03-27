pub struct Position {
    pub(crate) x : f32,
    pub(crate) y : f32,
}

impl Position {

    pub fn new(pos_x: f32, pos_y : f32) -> Self {
        Self {
            x: pos_x,
            y: pos_y,
        }
    }
}