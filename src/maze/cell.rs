use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct MazeCell {
    pub up: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl MazeCell {
    pub const fn new() -> Self {
        Self {
            up: true,
            left: true,
            right: true,
            down: true,
        }
    }
}

impl Display for MazeCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.up, self.left, self.right, self.down)
    }
}
