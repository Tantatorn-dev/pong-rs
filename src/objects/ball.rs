use ggez::{Context, GameResult, graphics::{self, Color, Mesh}};
use glam::Vec2;

pub struct Ball {
    pub pos_x: f32,
    pub pos_y: f32,
    pub direction: BallDirection,
}

pub struct BallDirection {
    pub v_direction: VerticalDirection,
    pub h_direction: HorizontalDirection,
}

pub enum VerticalDirection {
    UP,
    DOWN,
}
pub enum HorizontalDirection {
    LEFT,
    RIGHT,
    STILL,
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

    pub fn move_ball(&mut self) {

        match self.direction.v_direction {
            VerticalDirection::UP => self.pos_y -= 5.0,
            VerticalDirection::DOWN => self.pos_y += 5.0,
        }

        match self.direction.h_direction {
            HorizontalDirection::LEFT => self.pos_x -= 5.0,
            HorizontalDirection::RIGHT => self.pos_x += 5.0,
            HorizontalDirection::STILL => (),
        }

    }

}
