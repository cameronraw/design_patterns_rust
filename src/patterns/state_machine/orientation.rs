pub enum Orientation {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl Orientation {
    pub fn get_char(&self) -> char {
        match self {
            Orientation::NORTH => 'N',
            Orientation::EAST => 'E',
            Orientation::SOUTH => 'S',
            Orientation::WEST => 'W'
        }
    }
}