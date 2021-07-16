pub struct PaddleState {
    pub pos_x: f32,
    pub pos_y: f32,
    pub direction: PaddleDirection,
}

pub enum PaddleDirection {
    LEFT,
    RIGHT,
    STILL,
}
