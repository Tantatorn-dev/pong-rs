pub struct BallState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub direction: BallDirection,
}

pub enum BallDirection {
    NORTH,
    SOUTH,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}