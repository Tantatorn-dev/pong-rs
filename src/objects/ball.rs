use ggez::{Context, GameResult, graphics::{self, Color, Mesh, Rect}};
use glam::Vec2;

pub struct Ball {
    pos_x: f32,
    pos_y: f32,
    direction: BallDirection,
}

pub enum BallDirection {
    NORTH,
    SOUTH,
    NORTHEAST,
    NORTHWEST,
    SOUTHEAST,
    SOUTHWEST,
}

impl Ball {
    
    pub fn new(pos_x: f32, pos_y: f32, direction: BallDirection) -> Self {
        return Ball {
            pos_x: pos_x,
            pos_y: pos_y,
            direction: direction,
        }
    }

	pub fn get_mesh(&mut self, ctx: &mut Context) -> GameResult<Mesh> {
        return graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(self.pos_x, self.pos_y),
            20.0,
            2.0,
            Color::WHITE,
        );
	}

}
